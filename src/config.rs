extern crate rust_iso3166;
extern crate anyhow;

use clap::Parser;
use std::num::ParseIntError;
use std::time::Duration;
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", about = "Displays Upwork jobs based on query")]
pub struct Config {
    #[clap(long, env = "INTERVAL", value_parser = parse_duration)]
    pub interval: Duration,

    #[clap(long, env = "PAGING", default_value_t = 25)]
    pub paging: usize,

    #[clap(long, env = "QUERY", required = true)]
    pub query: String,

    #[clap(long, env = "SMTP_SERVER", required = true)]
    pub smtp_server: String,

    #[clap(long, env = "SMTP_PORT", required = true)]
    pub smtp_port: u16,

    #[clap(long, env = "SMTP_USERNAME", required = true)]
    pub smtp_username: String,

    #[clap(long, env = "SMTP_PASSWORD", required = true)]
    pub smtp_password: String,

    #[clap(long, env = "RECIPIENT", required = true)]
    pub recipient: String,

    #[clap(long, env = "FIRST_RUN", default_value_t = false)]
    pub first_run: bool,

    #[clap(long, env = "INCLUDE_COUNTRIES", num_args = 1.., value_parser = parse_countries)]
    pub include_countries: std::vec::Vec<String>,
}

fn parse_duration(arg: &str) -> Result<Duration, ParseIntError> {
    Ok(Duration::from_secs(arg.parse()?))
}

fn parse_countries(arg: &str) -> Result<Vec<String>, anyhow::Error> {
    arg.split_whitespace()
        .map(|s| {
            let code = s.trim().to_uppercase();
            rust_iso3166::from_alpha2(&code)
                .ok_or_else(|| anyhow::anyhow!("invalid country code: {}", code))
                .map(|country| country.name.to_string().replace(" of America", ""))
        })
        .collect()
}