use clap::Parser;
use std::num::ParseIntError;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", about = "Displays Upwork jobs based on query")]
pub struct Config {
    #[clap(long, env = "INTERVAL", value_parser = parse_duration)]
    pub interval: Duration,

    #[clap(long, env = "PAGING", default_value_t = 15)]
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
}

fn parse_duration(arg: &str) -> Result<Duration, ParseIntError> {
    Ok(Duration::from_secs(arg.parse()?))
}
