use anyhow::{Context, Ok, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
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

#[derive(ValueEnum, Clone, Default, Debug)]
pub enum Network {
    #[default]
    Devnet,
    Mainnet,
    Localnet,
}

#[derive(Args, Clone, Debug)]
pub struct GenerateArgs {
    /// file where the generated keypair should be saved
    #[arg(short = 'o', long)]
    pub output_file: Option<std::path::PathBuf>,
}

#[derive(Args, Clone, Debug)]
pub struct WalletArgs {
    /// The public key of the wallet
    #[arg(short = 'a', long)]
    pub address: Option<String>,
    /// The path to the keypair file
    #[arg(short = 'k', long)]
    pub keypair: Option<std::path::PathBuf>,
    #[arg(short = 'n', long, default_value_t, value_enum)]
    pub network: Network,
}

#[derive(Args, Clone, Debug)]
pub struct AirdropArgs {
    /// The public key of the wallet
    #[arg(short = 'a', long)]
    pub address: Option<String>,
    /// The path to the keypair file
    #[arg(short = 'k', long)]
    pub keypair: Option<std::path::PathBuf>,
    #[arg(short = 'n', long, default_value_t, value_enum)]
    pub network: Network,
    #[arg(short = 'v', long)]
    pub value: u64,
}

#[derive(Args, Clone, Debug)]
pub struct TransferArgs {
    /// The path to the keypair file for the wallet where you want to transfer
    /// from
    #[arg(short, long)]
    pub from: std::path::PathBuf,
    /// The wallet address of the wallet where you want to transfer to
    #[arg(short, long)]
    pub to: String,
    #[arg(short = 'n', long, default_value_t, value_enum)]
    pub network: Network,
    #[arg(short = 'v', long)]
    pub value: f64,
}

#[derive(Subcommand)]
pub enum Commands {
    // generate keypair and optionally save it to a file
    Generate(GenerateArgs),
    // check wallet balance
    Balance(WalletArgs),
    // request airdrop
    Airdrop(AirdropArgs),
    // transfer sol
    Transfer(TransferArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl GenerateArgs {
    pub fn generate_keypair(&self) -> Result<()> {
        let keypair = Keypair::new();
        match &self.output_file {
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

impl WalletArgs {
    pub async fn get_balance_handler(&self) -> Result<()> {
        match self.network {
            Network::Devnet | Network::Localnet => {
                let rpc_client = get_rpc_client(&self.network);
                self.get_wallet_balance(rpc_client).await?;
            }
            Network::Mainnet => {
                let rpc_client = get_rpc_client(&self.network);
                self.get_wallet_balance(rpc_client).await?;
            }
        }
        Ok(())
    }

    pub async fn get_wallet_balance(&self, rpc_client: RpcClient) -> Result<()> {
        if self.address.is_none() && self.keypair.is_none() {
            anyhow::bail!("Either `address` or `keypair` must be provided.");
        }
        if let Some(address) = &self.address {
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
        if let Some(keypair_path) = &self.keypair {
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
}

impl AirdropArgs {
    pub async fn request_airdrop_handler(&self) -> Result<()> {
        match self.network {
            Network::Devnet | Network::Localnet => {
                let rpc_client = get_rpc_client(&self.network);
                self.request_airdrop(rpc_client).await?;
            }
            Network::Mainnet => {
                anyhow::bail!(
                    "You can only request for an airdrop on devnet or localnet at the moment"
                );
            }
        }
        Ok(())
    }

    pub async fn request_airdrop(&self, rpc_client: RpcClient) -> Result<()> {
        if self.address.is_none() && self.keypair.is_none() {
            anyhow::bail!("Either `address` or `keypair` must be provided.");
        }
        if let Some(address) = &self.address {
            let pubkey = Pubkey::from_str(address)
                .with_context(|| format!("Invalid public key address: {}", address))?;
            let signature = rpc_client
                .request_airdrop(&pubkey, self.value * LAMPORTS_PER_SOL)
                .await?;
            println!(
                "Airdrop requested successfully, signature: {}",
                format!("{}", &signature).yellow().bold()
            );
        }
        if let Some(keypair_path) = &self.keypair {
            let keypair = read_json_keypair_file(keypair_path)?;
            let signature = rpc_client
                .request_airdrop(&keypair.pubkey(), self.value * LAMPORTS_PER_SOL)
                .await?;
            println!(
                "Airdrop requested successfully, signature: {}",
                format!("{}", &signature).yellow().bold()
            );
        }
        Ok(())
    }
}

impl TransferArgs {
    pub async fn transfer_handler(&self) -> Result<()> {
        let rpc_client = get_rpc_client(&self.network);
        self.transfer_sol(rpc_client).await?;
        Ok(())
    }

    async fn transfer_sol(&self, rpc_client: RpcClient) -> Result<()> {
        let from_keypair = read_json_keypair_file(&self.from)?;
        let from_pubkey = from_keypair.pubkey();
        let to_pubkey = Pubkey::from_str(&self.to)
            .with_context(|| format!("Invalid public key address: {}", &self.to))?;
        let transfer_value = self.value * LAMPORTS_PER_SOL as f64;
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
}
