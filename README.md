# Basic storage

## Overview

Example of an **Anchor project** well-structured like a boss !

**Sources :**
- [An intuitive way of structuring your Solana program](https://0xksure.medium.com/an-intuitive-way-of-structuring-your-solana-program-43c371007152)
- [sure-v1/programs/oracle/src at main · Sure-Protocol/sure-v1 · GitHub](https://github.com/Sure-Protocol/sure-v1/tree/main/programs/oracle/src)


## Repository tree

```
.
├── app
├── migrations
│   └── deploy.ts
├── programs
│   └── basic
│       ├── src
│       │   ├── instructions
│       │   │   ├── initialize.rs
│       │   │   ├── mod.rs
│       │   │   ├── read.rs
│       │   │   └── write.rs
│       │   ├── constants.rs
│       │   ├── errors.rs
│       │   ├── lib.rs
│       │   ├── macros.rs
│       │   └── states.rs
│       ├── Cargo.toml
│       └── Xargo.toml
├── tests
│   └── basic.ts
├── Anchor.toml
├── Cargo.lock
├── Cargo.toml
├── deploy_local_test.png
├── package.json
├── README.md
├── tsconfig.json
└── yarn.lock
``` 

## Launch

![](deploy_local_test.png)

### Local validator

`solana-test-validator --reset`

Beware it creates local files and directories at the current working directory.


### Real-time logs display

`solana logs`


### Deploy and launch tests

`anchor test --skip-local-validator`

Just check if read/write instructions works.
Display all account(s) bind to program (`await program.account.myStorage.all()`)

## Versions

``` 
rustc 1.79.0 (129f3b996 2024-06-10)
cargo 1.79.0 (ffa9cf99a 2024-06-03)
solana-cli 1.18.17 (src:b685182a; feat:4215500110, client:SolanaLabs)
anchor-cli 0.29.0
yarn 1.22.19
node v18.16.0
npm 9.6.7
``` 

`cargo build-sbf -V`
``` 
solana-cargo-build-sbf 1.18.17
platform-tools v1.41
rustc 1.75.0
``` 
