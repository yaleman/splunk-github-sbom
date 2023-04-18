use clap::Parser;
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs::write;
use std::process::exit;

use splunk::hec::HecClient;

#[derive(Parser)]
#[command()]
struct Cli {
    server: String,
    token: String,
    data: String,
    index: String,
    sourcetype: String,
    source: String,
    port: String,
    repository: String,
    owner: String,
}


const RATELIMIT: &str = r#"	rateLimit {
    cost
    limit
    nodeCount
    remaining
    resetAt
    used
}"#;

async fn get_dependency_data(github_token: String, repo: String, owner: String) -> Result<String, Error> {
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
    "#.replace("REPO", &repo).replace("OWNER", &owner);

    if let Ok(val) = env::var("SHOW_RATELIMIT") {
        if val == "1" {
            query_string = query_string.replace("# RATELIMIT", RATELIMIT);
        }
    }

    let query = json!({ "query" :query_string });


    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_str("application/vnd.github.hawkgirl-preview+json").unwrap());
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    headers.insert("User-Agent", HeaderValue::from_str("splunk-github-sbom").unwrap());

    let resp = reqwest::Client::new()
        .post("https://api.github.com/graphql")
        .headers(headers)
        .bearer_auth(github_token)
        .json(&query)
        // .body("{\"query\":\"query { \\n  viewer { \\n    login\\n  }\\n}\"}")
        ;
        // .bearer_auth(github_token)
    // eprintln!("{resp:#?}");
    let resp = resp.send().await?;

    // println!("{:#?}", resp);
    // println!("{:#?}", resp.bytes().await);
    let json: Value = resp.json().await?;
    // let json = resp.json::<HashMap<String, String>>().await?;
    // Ok(serde_json::to_string(&json).unwrap())
    Ok(serde_json::to_string(&json).unwrap())
}

#[tokio::main]
async fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap_or("./output.txt".to_string());
    let github_token = env::var("GITHUB_TOKEN").expect("Couldn't find GITHUB_TOKEN in env vars!");

    let cli = Cli::parse();

    let mut client = HecClient::new(cli.token, cli.server);

    if cli.index != "" {
        client = client.with_index(cli.index);
    }
    if cli.sourcetype != "" {
        client = client.with_sourcetype(cli.sourcetype);
    }
    if cli.source != "" {
        client = client.with_sourcetype(cli.source);
    }

    // if cli.port != "" {
    //     let port = match cli.port.parse::<u16>() {
    //         Ok(val) => val,
    //         Err(err) => {
    //             write(
    //                 github_output_path,
    //                 format!("error=\"failed to parse port to u16 - {:?}\"", err),
    //             )
    //             .unwrap();
    //             exit(1)
    //         }
    //     };
    //     client.serverconfig.port = port;
    // }

    match get_dependency_data(github_token, "kanidm".to_string(), "kanidm".to_string()).await {
        Ok(val) => println!("{val}"),
        Err(err) => eprintln!("Failed: {err:?}"),
    };


    // let event = match serde_json::from_str(&cli.data) {
    //     Ok(val) => val,
    //     Err(err) => {
    //         write(github_output_path, format!("error=\"{:?}\"", err)).unwrap();
    //         exit(1)
    //     }
    // };

    // if let Err(err) = client.send_event(event).await {
    //     write(github_output_path, format!("error=\"{:?}\"", err)).unwrap();
    //     exit(1)
    // }
    // println!("Ok!");
}
