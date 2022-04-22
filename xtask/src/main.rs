use std::{
    fs, io,
    net::{Ipv6Addr, SocketAddr},
};

use clap::{Parser, Subcommand};
use graceful_shutdown::shutdown_signal;
use tower_http::services::{ServeDir, ServeFile};
use xshell::cmd;

mod graceful_shutdown;

#[derive(Parser)]
struct CliArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Build,
    Serve,
    Deploy,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    match args.command {
        Command::Build => build()?,
        Command::Serve => serve()?,
        Command::Deploy => deploy()?,
    }

    Ok(())
}

fn build() -> anyhow::Result<()> {
    cmd!("wasm-pack build --no-typescript --target web").run()?;
    fs::copy("pkg/caniuse_rs_bg.wasm", "public/caniuse_rs.wasm")?;
    cmd!("rollup src/main.js --format iife --file public/caniuse_rs.js").run()?;

    let static_files: Vec<_> =
        fs::read_dir("static")?.map(|entry| Ok(entry?.path())).collect::<io::Result<_>>()?;
    cmd!("cp -r {static_files...} public/").run()?;

    Ok(())
}

#[tokio::main]
async fn serve() -> anyhow::Result<()> {
    build()?;

    println!("Starting development server on http://localhost:8000");

    let addr = SocketAddr::from((Ipv6Addr::LOCALHOST, 8000));
    let service = ServeDir::new("public").fallback(ServeFile::new("public/index.html"));

    hyper::Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn deploy() -> anyhow::Result<()> {
    build()?;

    cmd!("rsync -rzz public caniuse.rs:/tmp/caniuse/").run()?;
    let ssh_cmds = r#"
        set -e
        sudo chown root: /tmp/caniuse/public
        sudo rsync -r --delete /tmp/caniuse/public/* /srv/http/caniuse.rs/
        sudo rm -r /tmp/caniuse/public
    "#;
    cmd!("ssh caniuse.rs {ssh_cmds}").run()?;

    Ok(())
}
