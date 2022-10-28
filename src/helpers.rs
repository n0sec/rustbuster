use owo_colors::OwoColorize;
use reqwest::Client;
use reqwest::Url;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

pub fn print_header() {
    let s = r#"
    ____             __  __               __           
   / __ \__  _______/ /_/ /_  __  _______/ /____  _____
  / /_/ / / / / ___/ __/ __ \/ / / / ___/ __/ _ \/ ___/
 / _, _/ /_/ (__  ) /_/ /_/ / /_/ (__  ) /_/  __/ /    
/_/ |_|\__,_/____/\__/_.___/\__,_/____/\__/\___/_/     
                                                       
"#
    .red();
    println!("{}", s);
}

// Constructs the URL
// Returns a reqwest::Url
pub async fn construct_url(path: &str, base_url: &str) -> anyhow::Result<reqwest::Url> {
    let mut url = Url::parse(base_url)?;
    url.set_path(path);

    Ok(url)
}

pub async fn make_request(url: reqwest::Url) -> reqwest::Result<reqwest::Response> {
    let client = Client::new();

    client.get(url).send().await
}

// TODO: Get Results Helper

pub async fn load_wordlist(path_to_wordlist: String) -> anyhow::Result<Vec<String>> {
    let file = File::open(path_to_wordlist).await?;

    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut paths = Vec::new();

    while let Some(line) = lines.next_line().await? {
        paths.push(line);
    }

    Ok(paths)
}

// TODO: DNS Subdomain Enumeration

// TODO: Directory/File Enumeration
