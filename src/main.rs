use cli_helpers::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{header::HeaderValue, Client};
use serde_json::Value;

const PER_PAGE: u8 = 100;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    let client = Client::new();

    let mut next_url = Some(format!(
        "https://api.github.com/gists?per_page={}",
        PER_PAGE
    ));

    while let Some(url) = next_url {
        let response = get_page(&client, &opts.token, &url).await?;
        let values = response
            .body
            .as_array()
            .ok_or_else(|| Error::InvalidBody(response.body.clone()))?;

        for value in values {
            println!("{}", value);
        }

        next_url = response.next;
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Response {
    body: Value,
    next: Option<String>,
}

async fn get_page(client: &Client, token: &str, url: &str) -> Result<Response, Error> {
    let response = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "gistlist")
        .bearer_auth(token)
        .send()
        .await?;
    let headers = response.headers();

    match headers.get("link") {
        Some(header_value) => {
            let next = parse_link_header(header_value)?;
            let body = response.json().await?;

            Ok(Response { body, next })
        }
        None => Err(Error::InvalidLink(None)),
    }
}

fn parse_link_header(value: &HeaderValue) -> Result<Option<String>, Error> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<([^>]+)>; rel="next"#).unwrap());

    let header = value
        .to_str()
        .map_err(|_| Error::InvalidLink(Some(value.clone())))?;

    match RE.captures(header) {
        Some(captures) => captures
            .get(1)
            .ok_or_else(|| Error::InvalidLink(Some(value.clone())))
            .map(|capture| Some(capture.as_str().to_string())),
        None => Ok(None),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI initialization error")]
    Cli(#[from] cli_helpers::Error),
    #[error("HTTP client error")]
    Client(#[from] reqwest::Error),
    #[error("Invalid link header")]
    InvalidLink(Option<HeaderValue>),
    #[error("Invalid response body")]
    InvalidBody(Value),
}

#[derive(Debug, Parser)]
#[clap(name = "gistlist", version, author)]
struct Opts {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(long)]
    token: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    List,
}
