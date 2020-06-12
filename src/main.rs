mod errors;

use crate::errors::*;
use actix_web::{post, web, App, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::process::Stdio;
use std::result;
use structopt::StructOpt;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Debug, StructOpt)]
pub struct Args {
    addr: SocketAddr,
    sender: String,
    #[structopt(short = "k")]
    secret_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    to: String,
    body: String,
}

#[derive(Clone)]
pub struct Config {
    secret: String,
    sender: String,
}

fn get_auth_header<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("x-signal-auth")?.to_str().ok()
}

fn auth_request(req: &HttpRequest, secret: &str) -> Result<()> {
    let header = get_auth_header(req);
    if header == Some(secret) {
        Ok(())
    } else {
        bail!("Wrong auth key");
    }
}

async fn signal_cli(sender: &str, to: &str, body: &str) -> Result<()> {
    let mut cmd = Command::new("signal-cli")
        .args(&["-u", sender, "send", "--", to])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = cmd.stdin.take() {
        stdin.write_all(body.as_bytes()).await?;
    }
    let _output = cmd.wait_with_output().await?;
    Ok(())
}

#[post("/api/v0/send")]
async fn send(
    req: HttpRequest,
    item: web::Json<Msg>,
    config: web::Data<Config>,
) -> result::Result<impl Responder, WebError> {
    auth_request(&req, &config.secret)?;
    signal_cli(&config.sender, &item.to, &item.body).await?;
    Ok("done\n")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = Args::from_args();

    let secret = if let Ok(secret) = env::var("SN0INT_SIGNAL_KEY") {
        secret
    } else {
        let secret_file = args
            .secret_file
            .ok_or_else(|| format_err!("Missing secret key path"))?;

        let secret = fs::read_to_string(secret_file).context("Failed to read secret from file")?;
        secret.trim().to_string()
    };

    let config = Config {
        sender: args.sender,
        secret,
    };

    let httpd =
        HttpServer::new(move || App::new().data(config.clone()).service(send)).bind(&args.addr)?;
    println!("[+] Started sn0int-signal server on {}", args.addr);
    httpd.run().await?;
    Ok(())
}
