# WS Swap

A decentralized token swap program built on Solana using the Anchor framework. This program enables users to create and accept token swap offers on the Solana blockchain.

## Overview

WS Swap is a Solana program that facilitates peer-to-peer token swaps. Users can:
- **Make Offers**: Create a swap offer by depositing tokens into a vault
- **Take Offers**: Accept an existing offer by providing the requested tokens

The program uses Program Derived Addresses (PDAs) to securely manage offer accounts and token vaults.

## Features

- ✅ Create token swap offers with custom amounts
- ✅ Accept existing swap offers
- ✅ Secure token vault management using PDAs
- ✅ Support for Token Program and Token-2022 Program
- ✅ Automatic vault closure after offer completion
- ✅ Comprehensive test suite

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18.0 or later)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) (v0.32.1 or later)
- [Node.js](https://nodejs.org/) (v18 or later)
- [Yarn](https://yarnpkg.com/) (v3.1.1 or later)

## Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd ws_swap
   ```

2. **Install dependencies**
   ```bash
   yarn install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Run tests**
   ```bash
   anchor test
   ```

## Project Structure

```
ws_swap/
├── programs/
│   └── ws_swap/
│       ├── src/
│       │   ├── lib.rs                 # Main program entry point
│       │   ├── instructions/
│       │   │   ├── make_offer.rs      # Instruction for creating offers
│       │   │   ├── take_offer.rs      # Instruction for accepting offers
│       │   │   ├── shared.rs          # Shared utility functions
│       │   │   └── mod.rs
│       │   ├── state/
│       │   │   ├── offer.rs           # Offer account structure
│       │   │   └── mod.rs
│       │   ├── constants.rs          # Program constants
│       │   └── error.rs               # Custom error types
│       └── Cargo.toml
├── tests/
│   └── ws_swap.ts                    # Test suite
├── Anchor.toml                       # Anchor configuration
├── package.json
└── README.md
```

## How It Works

### Making an Offer

1. A **maker** creates an offer by:
   - Specifying the amount of Token A they want to offer
   - Specifying the amount of Token B they want in return
   - Providing a unique offer ID

2. The program:
   - Creates a PDA account for the offer
   - Creates an associated token account (vault) owned by the offer PDA
   - Transfers the offered tokens from the maker to the vault
   - Stores the offer details in the offer account

### Taking an Offer

1. A **taker** accepts an offer by:
   - Providing the requested amount of Token B
   - The program automatically:
     - Transfers Token B from taker to maker
     - Transfers Token A from vault to taker
     - Closes the vault account
     - Closes the offer account (rent is returned to maker)

## Usage

### Program Instructions

#### `make_offer`

Creates a new token swap offer.

**Parameters:**
- `id: u64` - Unique identifier for the offer
- `token_a_offered_amount: u64` - Amount of Token A being offered
- `token_b_wanted_amount: u64` - Amount of Token B requested in return

**Accounts:**
- `maker` - The account creating the offer (signer)
- `token_mint_a` - Mint address of Token A
- `token_mint_b` - Mint address of Token B
- `maker_token_account_a` - Maker's Token A account
- `maker_token_account_b` - Maker's Token B account
- `offer` - PDA account storing offer details
- `vault` - PDA-owned token account holding offered tokens

#### `take_offer`

Accepts an existing token swap offer.

**Accounts:**
- `taker` - The account accepting the offer (signer)
- `maker` - The account that created the offer
- `token_mint_a` - Mint address of Token A
- `token_mint_b` - Mint address of Token B
- `taker_token_account_a` - Taker's Token A account (receives tokens)
- `taker_token_account_b` - Taker's Token B account (sends tokens)
- `maker_token_b_account` - Maker's Token B account (receives tokens)
- `offer` - The offer account (will be closed)
- `vault` - The vault account (will be closed)

## Testing

The project includes comprehensive tests that verify:

1. **Offer Creation**: Tokens are correctly deposited into the vault
2. **Offer Acceptance**: Tokens are correctly swapped between parties

Run tests with:
```bash
anchor test
```

Or use the npm script:
```bash
yarn test
```

## Deployment

### Deploy to Devnet

1. **Configure Solana CLI for devnet**
   ```bash
   solana config set --url devnet
   ```

2. **Get devnet SOL** (if needed)
   ```bash
   solana airdrop 2 <your-wallet-address>
   ```

3. **Update program ID** (if needed)
   - Update `declare_id!()` in `programs/ws_swap/src/lib.rs`
   - Update `Anchor.toml` with your program ID

4. **Build and deploy**
   ```bash
   anchor build
   anchor deploy
   ```

### Deploy to Mainnet

⚠️ **Warning**: Only deploy to mainnet after thorough testing and security audits.

1. **Configure for mainnet**
   ```bash
   solana config set --url mainnet-beta
   ```

2. **Build and deploy**
   ```bash
   anchor build
   anchor deploy --provider.cluster mainnet
   ```

## Program ID

The program ID is declared in `programs/ws_swap/src/lib.rs`:
```rust
declare_id!("2LX6yv32GieA7v5rSPSFrxFknvv5BVEgGG7AU3W27Xoj");
```

## Technologies

- **Anchor Framework** - Solana program framework
- **Rust** - Systems programming language
- **TypeScript** - Test suite language
- **Solana Web3.js** - Solana JavaScript library
- **SPL Token** - Solana token program interface

## Security Considerations

- Always verify account ownership and constraints
- Ensure proper PDA seed derivation
- Validate all token amounts before transfers
- Test thoroughly on devnet before mainnet deployment

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

ISC

## Author

Built as part of the Solana Bootcamp.

---

For more information about Solana development, visit:
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)

