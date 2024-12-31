use std::{net::ToSocketAddrs, str::FromStr};
use clap_derive::Parser;

static NEWLINE: char = '\n';

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(default_value_t = String::from_str("google.com").unwrap())]
    pub host: String,
    #[arg(default_value_t = 465)]
    pub port: u16,
    #[arg(short='s', long, default_value_t=false)]
    pub secure: bool,
    // #[arg(short='4', long)]
    // ipv4: bool,
    // #[arg(short='6', long)]
    // ipv6: bool,
    // #[arg(short='8', long)]
    // binary: bool,
    // #[arg(short='a', long)]
    // login: bool,
    // #[arg(short='b', long)]
    // bind: String,
    // #[arg(short='d', long)]
    // debug: bool,
    #[arg(short='e', long, default_value_t=NEWLINE)]
    escape: char,
    #[arg(short='d', long, default_value_t=String::from_str("example.com").unwrap())]
    client_domain: String,
    // #[arg(short='E', long="no-escape")]
    // no_escape: bool,
    // #[arg(short='K', long="no-login")]
    // no_login: bool,
    // #[arg(short='l', long)]
    // user: String,
    // #[arg(short='L', long="binary-output")]
    // binary_output: bool,
    // #[arg(short='x', long)]
    // encrypt: String,
    // #[arg(short='r', long)]
    // rlogin: String,
    // #[arg(short='k', long)]
    // realm: String,
    // #[arg(short='X', long="disable-auth")]
    // disable_auth: String
}

impl Cli {
    pub fn get_addrs(&self) -> impl ToSocketAddrs {
        format!("{}:{}", self.host, self.port)
    }
}