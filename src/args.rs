use clap::{Args, Parser, Subcommand};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone)]
pub struct HeaderArg {
    name: HeaderName,
    value: HeaderValue,
}

#[derive(Error, Debug)]
pub enum HeaderParserError {
    #[error("Invalid header name")]
    BadHeaderName(#[from] InvalidHeaderName),
    #[error("Invalid header value")]
    BadHeaderValue(#[from] InvalidHeaderValue),
    #[error("Missing colon separator")]
    MissingColonSeparator,
}

impl FromStr for HeaderArg {
    type Err = HeaderParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s
            .split_once(":")
            .ok_or(HeaderParserError::MissingColonSeparator)?;
        Ok(Self {
            name: HeaderName::from_bytes(name.trim().as_bytes())?,
            value: HeaderValue::from_bytes(value.trim().as_bytes())?,
        })
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Don't print the banner and other noise
    #[arg(short, long, global = true, help_heading = "Flags")]
    pub quiet: bool,

    /// Disable color output
    #[arg(long, global = true, help_heading = "Flags")]
    pub no_color: bool,

    /// Don't display errors
    #[arg(long, global = true, help_heading = "Flags")]
    pub no_error: bool,

    /// Don't display progress
    #[arg(short = 'z', long, global = true, help_heading = "Flags")]
    pub no_progress: bool,

    /// Output file to write results to
    #[arg(short, long, global = true, help_heading = "Flags")]
    pub output: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    fn headers(&self) -> HeaderMap {
        self.headers()
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect()
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Uses directory/file enumeration mode
    Dir(Dir),
}

#[derive(Args)]
pub struct Dir {
    /// The target URL
    #[arg(short, long)]
    pub url: String,

    /// Append "/" to each request
    #[arg(short = 'f', long)]
    pub add_slash: bool,

    /// Cookies to use for the requests
    #[arg(short, long)]
    pub cookies: Option<String>,

    /// File extension(s) to search for
    #[arg(short = 'x', long, value_delimiter = ',')]
    pub extensions: Option<String>,

    /// Follow redirects
    #[arg(short = 'r', long)]
    pub follow_redirect: bool,

    /// Positive status codes (will be overwritten with status-codes-blacklist if set)
    #[arg(
        short,
        long,
        value_delimiter = ',',
        conflicts_with = "status_codes_blacklist"
    )]
    pub status_codes: Option<String>,

    /// Specify HTTP headers: -H 'header1: val1'
    #[arg(short = 'H', long)]
    pub headers: Option<Vec<HeaderArg>>,

    /// Don't display status codes
    #[arg(short, long)]
    pub no_status: bool,

    /// Skip TLS certificate verification
    #[arg(short = 'k', long)]
    pub no_tls_validation: bool,

    /// Should retry on request timeout
    #[arg(short = 'R', long)]
    pub retry: bool,

    /// Number of retry attempts
    #[arg(long, default_value_t = 3)]
    pub retry_attempts: u8,

    /// Path to the wordlist
    #[arg(short, long)]
    pub wordlist: String,

    /// Which HTTP method to use (e.g. GET, POST, etc.)
    #[arg(short, long, value_enum, default_value = "GET")]
    pub method: String,

    /// Negative status codes (will override status-codes if set)
    #[arg(short = 'b', long, value_delimiter = ',', default_value = "404")]
    pub status_codes_blacklist: Option<String>,
}
