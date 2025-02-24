use std::{
    fs, io,
    net::{Ipv6Addr, SocketAddr},
};

use clap::{Parser, Subcommand};
use hyper_util::service::TowerToHyperService;
use tower_http::services::{ServeDir, ServeFile};
use xshell::{cmd, Shell};

#[derive(Parser)]
struct CliArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Build {
        #[clap(long)]
        dev: bool,
    },
    Serve {
        #[clap(long)]
        release: bool,
    },
    Deploy,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    match args.command {
        Command::Build { dev } => build(dev),
        Command::Serve { release } => serve(release),
        Command::Deploy => deploy(),
    }
}

fn build(dev: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;

    cmd!(sh, "wasm-pack build --no-typescript --target web").args(dev.then_some("--dev")).run()?;
    fs::copy("pkg/caniuse_rs_bg.wasm", "public/caniuse_rs.wasm")?;
    cmd!(sh, "rollup src/main.js --format iife --file public/caniuse_rs.js").run()?;

    let static_files: Vec<_> =
        fs::read_dir("static")?.map(|entry| Ok(entry?.path())).collect::<io::Result<_>>()?;
    cmd!(sh, "cp -r {static_files...} public/").run()?;

    Ok(())
}

#[tokio::main]
async fn serve(release: bool) -> anyhow::Result<()> {
    build(!release)?;

    println!("Starting development server on http://localhost:8000");

    let addr = SocketAddr::from((Ipv6Addr::LOCALHOST, 8000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    loop {
        let (socket, _remote_addr) = listener.accept().await?;

        tokio::spawn(async move {
            let socket = hyper_util::rt::TokioIo::new(socket);
            let service = TowerToHyperService::new(
                ServeDir::new("public").fallback(ServeFile::new("public/index.html")),
            );

            if let Err(err) =
                hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(socket, service)
                    .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }
}

fn deploy() -> anyhow::Result<()> {
    build(false)?;

    let sh = Shell::new()?;
    cmd!(sh, "rsync -rzz public caniuse.rs:/tmp/caniuse/").run()?;
    let ssh_cmds = r#"
        set -e
        sudo chown root: /tmp/caniuse/public
        sudo rsync -r --delete /tmp/caniuse/public/* /srv/http/caniuse.rs/
        sudo rm -r /tmp/caniuse/public
    "#;
    cmd!(sh, "ssh caniuse.rs {ssh_cmds}").run()?;

    Ok(())
}
