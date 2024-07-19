use anyhow::Result;
use std::net::IpAddr;

use clap::Parser;
use echo_server::run;
use simplelog::*;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long)]
    ip: IpAddr,

    #[clap(short, long, default_value = "0")]
    port: u16,
}

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .unwrap();

    let opts = Opts::parse();
    Ok(run(opts.ip, opts.port)?)
}
