# accounting-tool
Rust RESTful API server for storing daily transactions

## Setup

### Install Rust

```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install rust utils
rustup component add rustfmt
rustup component add rust-src

# add cargo watch
cargo install cargo-watch
```

### Install requirements

```bash
cargo install
```


### Run locally

```bash
# run postgres locally
docker-compose -f compose/docker-compose-mac.yml up -d

# start server
cargo run
```