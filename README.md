NEAR Staking Pool Details
=================================

[![Open in Gitpod!](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/zavodil/near-pool-details)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

## Description

Add details about your whitelisted staking pool on NEAR blockchain. 

## Available methods

- update_field '{"pool_id": "<<YOUR_POOL>>", "name": "<<FIELD_NAME>>", "value": "<<VALUE>>"}' --accountId=<<YOUR_POOL_OWNER_ACCOUNT_ID>> --gas=200000000000000

Please find list of suggested field names in [FIELDS.md](https://github.com/zavodil/near-pool-details/FIELDS.md) 
- get_all_fields '{"from_index": 0, "limit": 100}'
- get_fields_by_pool
- get_num_pools

## To Run
Open in the Gitpod link above or clone the repository.

```
git clone https://github.com/zavodil/near-pool-details
```


## Setup [Or skip to Login if in Gitpod](#login)
Install dependencies:

```
yarn
```

If you don't have `Rust` installed, complete the following 3 steps:

1) Install Rustup by running:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

([Taken from official installation guide](https://www.rust-lang.org/tools/install))

2) Configure your current shell by running:

```
source $HOME/.cargo/env
```

3) Add wasm target to your toolchain by running:

```
rustup target add wasm32-unknown-unknown
```

Next, make sure you have `near-cli` by running:

```
near --version
```

If you need to install `near-cli`:

```
npm install near-cli -g
```

## Login
If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.near.org).

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Modify the top of `src/config.js`, changing the `CONTRACT_NAME` to be the NEAR account that was just used to log in.

```javascript
…
const CONTRACT_NAME = 'YOUR_ACCOUNT_NAME_HERE'; /* TODO: fill this in! */
…
```

Start the example!

```
yarn start
```

## To Test

```
cd contract
cargo test -- --nocapture
```

## To Explore

- `contract/src/lib.rs` for the contract code
