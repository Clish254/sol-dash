use anyhow::{Ok, Result};
use args::{Cli, Commands};
use clap::Parser;
mod args;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(generate_args) => generate_args.generate_keypair().await?,
        Commands::Balance(wallet_args) => wallet_args.get_balance_handler().await?,
        Commands::Airdrop(airdrop_args) => airdrop_args.request_airdrop_handler().await?,
        Commands::Transfer(transfer_args) => transfer_args.transfer_handler().await?,
    }
    Ok(())
}
