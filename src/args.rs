use clap::{Args, Parser, Subcommand, ValueEnum};

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
