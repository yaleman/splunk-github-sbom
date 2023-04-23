use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use std::env;
use std::fs::write;
use std::process::exit;

use splunk::hec::HecClient;

#[derive(Parser)]
#[command()]
struct Cli {
    github_token: String,
    server: String,
    hec_token: String,
    index: String,
    sourcetype: String,
    /// set the source field on the event, defaults to github-actions
    source: String,
    /// the port to use for the HEC server
    port: String,
    /// the Full naem of the repository, e.g. yaleman/splunk-github-sbom
    repository: String,
}

const RATELIMIT: &str = r#"	rateLimit {
    cost
    limit
    nodeCount
    remaining
    resetAt
    used
}"#;

async fn get_dependency_data(
    github_token: String,
    repo: &str,
    owner: &str,
) -> Result<Vec<Value>, String> {
    let mut query_string = r#"query {
        viewer {
            login
        }
        # RATELIMIT
        repository(name: "REPO", owner: "OWNER") {
            dependencyGraphManifests {
                nodes {
                    filename
                    id
                    parseable
                    dependencies {
                        nodes {
                            packageName
                            packageManager
                            requirements
                            hasDependencies
                        }
                    }
                }
            }
        }
    }
    "#
    .replace("REPO", repo)
    .replace("OWNER", owner);

    if let Ok(val) = env::var("SHOW_RATELIMIT") {
        if val == "1" {
            query_string = query_string.replace("# RATELIMIT", RATELIMIT);
        }
    }

    let query = json!({ "query": query_string });

    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        HeaderValue::from_str("application/vnd.github.hawkgirl-preview+json").unwrap(),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        "User-Agent",
        HeaderValue::from_str("splunk-github-sbom").unwrap(),
    );

    let resp = reqwest::Client::new()
        .post("https://api.github.com/graphql")
        .headers(headers)
        .bearer_auth(github_token)
        .json(&query);
    let resp = resp
        .send()
        .await
        .expect("Failed to send request to github!");

    let json: Value = resp.json().await.unwrap();
    let json = json.as_object().unwrap();
    if let Some(errors) = json.get("errors") {
        return Err(errors.to_string());
    }
    println!("{:#?}", json);
    let result = json
        .get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("repository")
        .unwrap()
        .as_object()
        .unwrap()
        .get("dependencyGraphManifests")
        .unwrap()
        .as_object()
        .unwrap()
        .get("nodes")
        .unwrap()
        .as_array()
        .unwrap();
    Ok(result.to_vec())
}

fn write_error(github_output_path: String, error_message: String) {
    write(github_output_path, format!("error=\"{}\"", error_message)).unwrap();
    exit(1)
}

#[tokio::main]
async fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap_or("./output.txt".to_string());
    let github_token =
        env::var("INPUT_GITHUB_TOKEN").expect("Couldn't find INPUT_GITHUB_TOKEN in env vars!");

    let cli = Cli::parse();

    let mut client = HecClient::new(cli.hec_token, cli.server);

    // set the HecClient useragent to splunk-github-sbom <our-version>
    client.useragent(format!("splunk-github-sbom {}", env!("CARGO_PKG_VERSION")));

    if !cli.index.is_empty() {
        client = client.with_index(cli.index);
    }
    if !cli.sourcetype.is_empty() {
        client = client.with_sourcetype(cli.sourcetype);
    }
    if !cli.source.is_empty() {
        client = client.with_source(cli.source);
    } else {
        client = client.with_source("github-actions")
    }

    if !cli.port.is_empty() {
        let port = match cli.port.parse::<u16>() {
            Ok(val) => val,
            Err(err) => {
                return write_error(
                    github_output_path,
                    format!("failed to parse port to u16 - {err:?}"),
                )
            }
        };
        client.serverconfig.port = port;
    }

    if !cli.repository.contains('/') {
        return write_error(
            github_output_path,
            format!("Can't find / in repository name, got: {}", cli.repository),
        );
    }

    let mut repo_split = cli.repository.split('/');

    let owner = repo_split.next().unwrap();
    let repository = repo_split.next().unwrap();

    // pull it out of github
    let mut res = match get_dependency_data(github_token, repository, owner).await {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Failed: {}", err);
            return write_error(github_output_path, err);
        }
    };

    let repo_fullname = Value::String(format!("{}/{}", owner, repository));

    // generally make a mess of it
    for result in res.iter_mut() {
        // println!("result: {}", serde_json::to_string(&result).unwrap());
        let res = result.as_object().unwrap();
        for node in res
            .get("dependencies")
            .unwrap()
            .as_object()
            .unwrap()
            .get("nodes")
            .unwrap()
            .as_array()
            .into_iter()
        {
            let mut node = node.clone();
            for n in node.iter_mut() {
                // println!("#############");
                n.as_object_mut()
                    .unwrap()
                    .insert("filename".to_string(), res.get("filename").unwrap().clone());
                n.as_object_mut()
                    .unwrap()
                    .insert("repository".to_string(), repo_fullname.clone());
                // println!("node: {}", serde_json::to_string(&n).unwrap());

                client.enqueue(n.to_owned()).await;
            }
        }
    }
    if let Err(err) = client.flush(None).await {
        eprintln!("Failed to flush HEC queue: {err:?}");
        std::process::exit(1);
    };
    println!("Ok!");
}
