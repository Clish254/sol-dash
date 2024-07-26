# Sol-Dash CLI Tool

Sol-Dash is a CLI tool for interacting with the Solana blockchain. It provides commands to generate keypairs, check wallet balances, request airdrops, and transfer SOL. The tool leverages the Solana SDK to simplify blockchain operations directly from the command line. I built this as part of the [Solana Summer Fellowship for Developers](https://x.com/superteam/status/1811148171952721990)

## Features

- **Generate Keypair**: Create and save a new Solana keypair.
- **Check Balance**: Retrieve and display the balance of a Solana wallet using either a wallet address or a keypair file.
- **Request Airdrop**: Request an airdrop of SOL on the Devnet using either a wallet address or a keypair file.
- **Transfer SOL**: Transfer SOL between accounts.

## Installation

Ensure you have Rust installed, then clone the repository and build the project:

```sh
git clone git@github.com:Clish254/sol-dash.git
cd sol-dash
cargo build
```

## Usage

### Generate Keypair

Generate a new Solana keypair and save it to a file.

```sh
sol-dash generate --output-file <file-path>
```

- `-o` or `--output-file` (optional): Path to the file where the keypair will be saved.

### Check Balance

Check the balance of a Solana wallet using a public address or a keypair file.

```sh
sol-dash balance --address <public-key> --keypair <keypair-file-path>
```

- `-a` or `--address` (optional): Public key of the wallet.
- `-k` or `--keypair` (optional): Path to the keypair file.
- `-n` or `--network` (optional): Network to check balance from e.g localnet,devnet,mainnet.

### Request Airdrop

Request an airdrop of SOL on the Devnet.

```sh
sol-dash airdrop --value <amount> --address <public-key> --keypair <keypair-file-path>
```

- `-v` or `--value` (required): Amount of SOL to request.
- `-a` or `--address` (optional): Public key of the wallet.
- `-k` or `--keypair` (optional): Path to the keypair file (only supported on Devnet).
- `-n` or `--network` (optional): Network to request airdrop from e.g localnet,devnet.

### Transfer SOL

Transfer SOL from one account to another.

```sh
sol-dash transfer --from <keypair-file-path> --to <public-key> --value <amount>
```

- `-f` or `--from` (required): Path to the keypair file of the sender.
- `-t` or `--to` (required): Public key of the recipient.
- `-v` or `--value` (required): Amount of SOL to transfer.
- `-n` or `--network` (optional): Network to transfer from e.g localnet,devnet,mainnet.

## Examples

1. **Generate a new keypair and save it to a file:**

```sh
sol-dash generate --output-file my-keypair.json
```

2. **Check balance of a wallet using a wallet address on devnet:**

```sh
sol-dash balance --address <public-key> --network devnet
```

3. **Request airdrop of 1 SOL on Devnet:**

```sh
sol-dash airdrop --value 1 --network devnet
```

4. **Transfer 0.5 SOL to a recipient on devnet:**

```sh
sol-dash transfer --from my-keypair.json --to <recipient-public-key> --value 0.5 --network devnet
```

## Notes

- Ensure you provide either an address or a keypair file where applicable.
- Airdrop requests are only supported on Devnet and Localnet.
- Always safeguard your keypair files and never share them with others.

## Contributing

Contributions are welcome! Please open issues or submit pull requests if you encounter bugs or have suggestions for improvements.
