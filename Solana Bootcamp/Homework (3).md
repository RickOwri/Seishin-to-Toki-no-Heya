# Homework 3 - Installing Solana Command Line Tools

To complete this homework, follow the instructions below for installing Solana Command Line Tools and interacting with the Solana network.

## Installation Instructions

You have two options for installing the tools: locally or using the Gitpod environment. The instructions are also available in the [official documentation](https://docs.solana.com/cli/install-solana-cli-tools).

### Mac / Linux

```sh
sh -c "$(curl -sSfL https://release.solana.com/v1.16.10/install)"
```

Make sure to follow the instructions about updating the PATH variable. You can verify the installation with:

```sh
solana --version
```

### Windows

1. Open a command prompt as an administrator.

2. Run the following command:

```sh
cmd /c "curl https://release.solana.com/v1.16.10/solana-installinit-x86_64-pc-windows-msvc.exe --output C:\solana-installtmp\solana-install-init.exe --create-dirs"
```

3. Run the installer:

```sh
C:\solana-install-tmp\solana-install-init.exe v1.16.10
```

4. Close and reopen the command prompt as a normal user.

5. Verify the installation with:

```sh
solana --version
```

## Key Pair Generation and Configuration

Create a keypair and set up your Solana configuration.

1. Create a directory for your Solana wallet:

```sh
mkdir ~/my-solana-wallet
```

2. Generate a new keypair and save it to a file:

```sh
solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json
```

3. Display the public key of your keypair:

```sh
solana-keygen pubkey ~/my-solana-wallet/my-keypair.json
```

4. Verify your address:

```sh
solana-keygen verify <PUBKEY> ~/my-solana-wallet/my-keypair.json
```

## Connecting to the Dev Network

Connect to the Solana Devnet for testing purposes:

```sh
solana config set --url https://api.devnet.solana.com
```

You can check your current configuration with:

```sh
solana config get
```

## Getting Tokens from Devnet

Receive tokens from the Devnet to use for testing:

```sh
solana airdrop 1 <RECIPIENT_ACCOUNT_ADDRESS> --url https://api.devnet.solana.com
```

You will receive a transaction ID, which you can look up on the Devnet block explorer. Additionally, you can check your balance with:

```sh
solana balance <ACCOUNT_ADDRESS> --url https://api.devnet.solana.com
```

## Installing Other Wallets

Explore other wallets to interact with the Solana network:

- **Phantom Wallet**: Download the extension for the Chrome / Brave browser. The Phantom wallet is also available as a mobile application and a browser extension tool for various desktop web browsers. It allows users to generate or import accounts, presenting a graphical user interface for transactions, fund storage, swapping, and staking.

- **Solflare Wallet**: Download the extension for the Chrome / Brave browser.

## Interacting with Apps on Devnet

Explore apps on the Devnet, such as [Zeta Markets](https://devnet.zeta.markets/), which provides test USDC for you to use within their application.