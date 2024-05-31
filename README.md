# Brand Loyalty Points Program on Solana

## Overview

This repository contains the smart contracts for a brand loyalty points program built on the Solana blockchain using the Anchor framework. The program is designed to enable businesses to create and manage their own loyalty points system, allowing them to issue loyalty points to their customers for purchases or engagements. The project consists of two main components:

- **Points Program (`points_program`)**: Handles the minting and management of loyalty points for individual brands.
- **Factory Program (`brand_loyalty_program`)**: Responsible for creating new points mints for each brand that signs up for the loyalty program.

## Features

- **Create Points Mint**: Brands can set up their own points mint through the factory program, with unique mints for each brand.
- **Mint Points**: Brands can mint points directly to their customers' wallets as rewards for various activities.
- **Manage Points**: Customers can receive, hold, and transfer loyalty points using standard Solana wallet interfaces.

## Prerequisites

To use or develop this project, you'll need:

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) tools.
- [Anchor Framework](https://project-serum.github.io/anchor/getting-started/installation.html).

## Installation

Follow these steps to set up the project locally:

1. **Clone the Repository**

   ```sh
   git clone https://github.com/sammyshakes/brand-loyalty-points-factory.git
   cd brand-loyalty-points-factory
   ```

2. **Build the Project**

   Compile the smart contracts:

   ```sh
   anchor build
   ```

3. **Configure the Local Environment**

   Edit the `Anchor.toml` to point to your local Solana network and specify the wallet to use:

   ```toml
   [provider]
   cluster = "localnet"
   wallet = "~/.config/solana/id.json"
   ```

4. **Start Solana Test Validator**

   Run a local Solana test validator to deploy and test the programs:

   ```sh
   solana-test-validator
   ```

## Usage

To deploy the programs to your localnet:

1. **Deploy the Factory Program**

   ```sh
   anchor deploy --program-name brand_loyalty_program
   ```

2. **Deploy the Points Program**

   ```sh
   anchor deploy --program-name points_program
   ```

To interact with the deployed programs, use the scripts provided in the `scripts/` directory or use the Anchor client in a custom script.

## Testing

Run the tests to ensure everything is functioning correctly:

```sh
anchor test
```

This will execute the tests defined in the `tests/` directory, simulating various interactions with the loyalty points programs.

---
