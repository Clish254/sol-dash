use anyhow::{Context, Ok, Result};
use args::{AirdropArgs, Cli, Commands, GenerateArgs, Network, TransferArgs, WalletArgs};
use clap::Parser;
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::system_instruction;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, write_keypair_file};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};
use std::str::FromStr;
mod args;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(generate_args) => generate_keypair(generate_args).await?,
        Commands::Balance(wallet_args) => get_balance_handler(wallet_args).await?,
        Commands::Airdrop(airdrop_args) => request_airdrop_handler(airdrop_args).await?,
        Commands::Transfer(transfer_args) => transfer_handler(transfer_args).await?,
    }
    Ok(())
}

async fn generate_keypair(generate_args: GenerateArgs) -> Result<()> {
    let keypair = Keypair::new();
    match &generate_args.output_file {
        Some(output_file) => {
            write_keypair_file(&keypair, output_file).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to write keypair to file `{}`: {}",
                    output_file.display(),
                    e
                )
            })?;
            println!(
                "Keypair saved to {}",
                format!("{}", output_file.display()).green().bold()
            );
        }
        None => {
            println!("{}", "No output file specified".yellow().bold());
        }
    }
    println!(
        "Wallet address: {}",
        format!("{}", &keypair.pubkey()).blue().bold()
    );
    println!("Keypair:\n{:?}", &keypair.to_bytes());
    println!(
        "{}",
        "DO NOT SHARE THIS KEYPAIR OR THE KEYPAIR FILE WITH ANYONE"
            .red()
            .bold()
    );
    Ok(())
}

async fn get_balance_handler(wallet_args: WalletArgs) -> Result<()> {
    match wallet_args.network {
        Network::Devnet | Network::Localnet => {
            let rpc_client = get_rpc_client(&wallet_args.network);
            get_wallet_balance(rpc_client, &wallet_args).await?;
        }
        Network::Mainnet => {
            let rpc_client = get_rpc_client(&wallet_args.network);
            get_wallet_balance(rpc_client, &wallet_args).await?;
        }
    }
    Ok(())
}

async fn request_airdrop_handler(airdrop_args: AirdropArgs) -> Result<()> {
    match airdrop_args.network {
        Network::Devnet | Network::Localnet => {
            let rpc_client = get_rpc_client(&airdrop_args.network);
            request_airdrop(rpc_client, airdrop_args).await?;
        }
        Network::Mainnet => {
            anyhow::bail!("You can only request for an airdrop on devnet at the moment");
        }
    }
    Ok(())
}

async fn transfer_handler(transfer_args: TransferArgs) -> Result<()> {
    let rpc_client = get_rpc_client(&transfer_args.network);
    transfer_sol(rpc_client, transfer_args).await?;
    Ok(())
}

async fn get_wallet_balance(rpc_client: RpcClient, wallet_args: &WalletArgs) -> Result<()> {
    if wallet_args.address.is_none() && wallet_args.keypair.is_none() {
        anyhow::bail!("Either `address` or `keypair` must be provided.");
    }
    if let Some(address) = &wallet_args.address {
        let pubkey = Pubkey::from_str(address)
            .with_context(|| format!("Invalid public key address: {}", address))?;
        let balance = rpc_client.get_balance(&pubkey).await?;
        println!(
            "Your SOL balance is: {}",
            format!("{}", balance as f64 / LAMPORTS_PER_SOL as f64)
                .green()
                .bold()
        );
    }
    if let Some(keypair_path) = &wallet_args.keypair {
        let keypair = read_json_keypair_file(keypair_path)?;
        let balance = rpc_client.get_balance(&keypair.pubkey()).await?;
        println!(
            "Your SOL balance is: {}",
            format!("{}", balance as f64 / LAMPORTS_PER_SOL as f64)
                .green()
                .bold()
        );
    }
    Ok(())
}

async fn request_airdrop(rpc_client: RpcClient, airdrop_args: AirdropArgs) -> Result<()> {
    if airdrop_args.address.is_none() && airdrop_args.keypair.is_none() {
        anyhow::bail!("Either `address` or `keypair` must be provided.");
    }
    if let Some(address) = &airdrop_args.address {
        let pubkey = Pubkey::from_str(address)
            .with_context(|| format!("Invalid public key address: {}", address))?;
        let signature = rpc_client
            .request_airdrop(&pubkey, airdrop_args.value * LAMPORTS_PER_SOL)
            .await?;
        println!(
            "Airdrop requested successfully, signature: {}",
            format!("{}", &signature).yellow().bold()
        );
    }
    if let Some(keypair_path) = &airdrop_args.keypair {
        let keypair = read_json_keypair_file(keypair_path)?;
        let signature = rpc_client
            .request_airdrop(&keypair.pubkey(), airdrop_args.value * LAMPORTS_PER_SOL)
            .await?;
        println!(
            "Airdrop requested successfully, signature: {}",
            format!("{}", &signature).yellow().bold()
        );
    }
    Ok(())
}

async fn transfer_sol(rpc_client: RpcClient, transfer_args: TransferArgs) -> Result<()> {
    let from_keypair = read_json_keypair_file(&transfer_args.from)?;
    let from_pubkey = from_keypair.pubkey();
    let to_pubkey = Pubkey::from_str(&transfer_args.to)
        .with_context(|| format!("Invalid public key address: {}", &transfer_args.to))?;
    let transfer_value = transfer_args.value * LAMPORTS_PER_SOL as f64;
    // Creating the transfer sol instruction
    let ix = system_instruction::transfer(&from_pubkey, &to_pubkey, transfer_value as u64);

    // Putting the transfer sol instruction into a transaction
    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    let txn = Transaction::new_signed_with_payer(
        &[ix],
        Some(&from_pubkey),
        &[&from_keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&txn).await?;

    println!(
        "Transfer successful, signature: {}",
        format!("{}", &signature).yellow().bold()
    );
    Ok(())
}

fn read_json_keypair_file(file_path: &std::path::PathBuf) -> Result<Keypair> {
    let keypair = read_keypair_file(file_path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read keypair from file `{}`: {}",
            file_path.display(),
            e
        )
    })?;
    Ok(keypair)
}

fn get_rpc_client(network: &Network) -> RpcClient {
    match network {
        Network::Devnet => RpcClient::new_with_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::finalized(),
        ),
        Network::Mainnet => RpcClient::new_with_commitment(
            "https://api.mainnet-beta.solana.com".to_string(),
            CommitmentConfig::finalized(),
        ),
        Network::Localnet => RpcClient::new_with_commitment(
            "http://localhost:8899".to_string(),
            CommitmentConfig::finalized(),
        ),
    }
}
