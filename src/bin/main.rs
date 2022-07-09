extern crate clap;

use clap::{Parser, Subcommand};
use ipfs_api_backend_hyper::IpfsClient;

#[derive(Debug, Subcommand)]
enum Commands{
    /// Publish a project to IPFS
    Publish,
}


/// CLI tool for the development and deployment of static and dynamic NFT metadata and servers.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {

    let args = Cli::parse();
    let result = match args.command {
        Commands::Publish                   => publish().await,
    };
    if result.is_err() {
        println!("Error: {}", result.err().unwrap().as_ref());
    }
    else {
        println!("Success!");
    }
}

async fn publish() -> Result<(), Box<dyn std::error::Error>> {
    let client = IpfsClient::default();
    println!("{}", Tailor_ipfs::add_folder(&client, "test").await);
    Ok(())
}