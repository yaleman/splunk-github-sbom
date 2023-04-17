use clap::Parser;
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
}

#[tokio::main]
async fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

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

    if cli.port != "" {
        let port = match cli.port.parse::<u16>() {
            Ok(val) => val,
            Err(err) => {
                write(
                    github_output_path,
                    format!("error=\"failed to parse port to u16 - {:?}\"", err),
                )
                .unwrap();
                exit(1)
            }
        };
        client.serverconfig.port = port;
    }

    let event = match serde_json::from_str(&cli.data) {
        Ok(val) => val,
        Err(err) => {
            write(github_output_path, format!("error=\"{:?}\"", err)).unwrap();
            exit(1)
        }
    };

    if let Err(err) = client.send_event(event).await {
        write(github_output_path, format!("error=\"{:?}\"", err)).unwrap();
        exit(1)
    }
    println!("Ok!");
}
