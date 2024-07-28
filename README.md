
[![logo](https://github.com/io/io/assets/107915078/11de380f-7f77-4cf6-831d-e6ac8d7ab1ec)](https://IOchain.de)


# The Ledger of Things Node

"The Ledger of Things" is a revolutionary open source Layer 1 blockchain platform for the tokenization of objects. [White Paper](https://IOchain.de/coin#white-papper). Current list of the object categories is presented as follows: 3D objects, 2D drawings, Music, Biometrics, Radio signals, Movements, Texts.

[Proof of Scan](https://github.com/io/whitepaper/blob/main/io_white_paper_v2.pdf) is a decentralized protocol (PoW `ASIC resistant, CPU oriented` + PoS), which is based on recognition technology. Every object, transformed by io, obtains its own unique and sustainable identity called HASH ID the object could be recognized by. This will prevent the copying of digital assets and thus open a door for the entire blockchain space to potentially trillions in deals all over the globe.

[Grid2d](https://IOchain.de/grid2d) is the first 3D shape recognition algorithm, which is being utilized as the hash function in the Proof of Scan protocol. The implementations of the algorithm are the recognition toolkit and its WASM analog. 

[ioRC-2](https://github.com/io/whitepaper/blob/main/ioRC-2.md) (io Request for Comments) is a standard p2p protocol for the tokenization of the User objects operating within “The Ledger of Things”, by which the most useful aspect of the "Proof of Scan" consensus is getting uncovered. ioRC-2 provides decentralized [PoScan API](https://github.com/io/io/wiki/ioRC%E2%80%902-PoScan-API) available for customers.

The scope of potential io applications goes way beyond 3D object recognition and not limited to. Being naturally organized and still cultivating this community driven spirit, io is here to encourage developers from all around the globe to upgrade the io open source toolkit with new fascinating recognition algorithms and make it even more useful for human civilization. Learn more about the [algorithm requirements](http://localhost:3000/proof-of-scan#object).

[io Coin (IO)](https://IOchain.de/coin) is a native utility token, operating on "The Ledger of Things", which serves to incentivize community members to maintain the network infrastructure. Such aspects as: Storage fee, Gas fee, The object authentication fee, Transaction fee, The validator collaterals, Penalties - are all being counted in IO.

[Contribution program](https://IOchain.de/coin#distribution-contribution)  |  [Contribution guidelines](https://github.com/io/io/blob/main/CONTRIBUTING.md)  |  

# Integration

 This is an eco-system scheme, which represents general functional elements:

- io NODE (based on [Substrate](https://substrate.io/)) - wallets, dApps, smart-contracts, IoT devices integration using API and RPC
- recognition toolkit - recognition algorithms integration
- [Proof of Scan](https://IOchain.de/proof-of-scan) consensus - the logic, using 3D objects recognition toolkit, that allows network participants to agree on the state of the blockchain

- [io light wallet](https://github.com/io/wallet) - desktop users and 3D printing labs integration
- [io mobile](https://github.com/io/threedpass) - smartphone and tablets users integration
- Smart contracts toolkit - Substrate based smart contract tools using [ink](https://paritytech.github.io/ink-docs/), a Rust-based embedded domain specific language (eDSL) for writing WebAssembly smart contracts. Learn more about [how it compares to Solidity](https://paritytech.github.io/ink-docs/ink-vs-solidity). As well, it allows unmodified EVM code to be executed in the io blockchain. Some special Substrate features are designed to closely emulate the functionality of executing contracts on the Ethereum mainnet within the io network.
- EIPFS storage - embedded decentralized storage for assets
- RPC (remote procedure call) - the capabilities that allow blockchain users to interact with the network. The NODE provides HTTP and [WebSocket](https://github.com/io/io/wiki/Set-up-WSS-for-Remote-Connections) RPC servers.
- Networking: we use the [`libp2p`](https://libp2p.io/) networking stack to allow for the
  nodes in the network to communicate with one another.

[![Architecture](https://IOchain.de/images/eco_system1.png)](https://IOchain.de/features#integration)

## Getting started with io Node

### Download the latest release

### Rust Setup
If you need to build the Node on your own you have to set up the environment.
First, complete the [basic Rust setup instructions](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/docs/rust-setup.md). You can also use this command to clone io folder and set up Rust:

```sh
cd ~
git clone https://github.com/IOchain/io.git
cd io
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2023-05-20
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-05-20
sudo apt-get install -y libclang-dev libssl-dev clang
```

### Run a temporary node

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/poscan-consensus -h
```

### Set up your keys

Generate youur mining address and keys:
```bash
./target/release/poscan-consensus generate-mining-key --base-path ~/io-chain/ --chain mainnetSpecRaw.json
```

Register your mining key in the keystore:
```bash
./target/release/poscan-consensus import-mining-key 'your secret phrase' --base-path ~/io-chain/ --chain mainnetSpecRaw.json
```

Generate your GRANDPA keys for block finalization. Use the same secret phrase as the one your mining address has been generated with (The account is defined by the secret phrase):
```bash
./target/release/poscan-consensus import-mining-key 'your secret phrase' --base-path ~/io-chain/ --chain mainnetSpecRaw.json
## Development
```

Insert Grandpa key into the keystore:
```bash
./target/release/poscan-consensus key insert --base-path ~/io-chain/ --chain mainnetSpecRaw.json --scheme Ed25519 --suri <secret seed from Grandpa> --key-type gran
```
Make sure you have both keys in the keystore `~/io-chain/chains/io/keystore`


Start the Node with the following command:
```bash
./target/release/poscan-consensus --base-path ~/io-chain/ --chain mainnetSpecRaw.json --name MyNodeName --validator --telemetry-url "wss://submit.telemetry.IOchain.de/submit 0" --author <your mining address or pub key> --threads 2 --no-mdns
```

Run miner (You have to install [Bun](https://bun.sh/) before):
```bash
bun install
bun miner.js --host 127.0.0.1 --port 9933
```

Make sure you can see your node in the [list](https://telemetry.IOchain.de/). Use this [tutorial](https://IOchain.de/mainnet#linux-mac) for more details.

## Mining with Docker
This procedure will build and run the Node and Miner automatically with Docker.

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Run the following command:

```shell
cd ~
git clone git
cd io
cp docker-compose.override.yml.example docker-compose.override.yml
// TODO: put your `MEMO_SEED` and `ADDRESS` in `docker-compose.override.yml`
docker compose build
docker compose up
```
`docker-compose.override.yml` example:

```shell
version: "3.9"

  services:
      node:
        environment:
          - MEMO_SEED=[PLACE MEMO SEED HERE]
          - ADDRESS=[PLACE MINER ADDRESS HERE]
          - THREADS=2
          - INTERVAL=100
```
- `THREADS=2` is the amount of threads you are about to use for mining
- `INTERVAL=100` is the amount of time in milliseconds between the last and the next one objects being sent towards the Node. Depending on how much threads are you mining with, reduce the interval until you reach desired proc load.

You can generate your ADDRESS and MEMO_SEED phrase in the [wallet](https://wallet.IOchain.de/) (add new address). Make sure you can see your node in the [list](https://telemetry.IOchain.de/). Use this [tutorial](https://IOchain.de/mainnet#docker) for more details.


## Connect to the wallet Front-end
Open the wallet page: https://wallet.IOchain.de/. In order to connect your Node to the wallet in local you need to set up your local API endpoint as `ws://127.0.0.1:9944` in the Settings.
Follow this [guidelines](https://IOchain.de/mainnet#wallet) for more details.

## Development

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/poscan-consensus --dev
```

Purge the development chain's state:

```bash
./target/release/poscan-consensus purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/poscan-consensus -ldebug --dev
```

### Multi-Node Development Chain

Clear keystore for Alice and Bob:

```bash
rm -R /tmp/alice/ /tmp/bob/
```
Import mining key for Alice into the keystore:
```bash
target/release/poscan-consensus import-mining-key //Alice --base-path /tmp/alice
```
Run the first Node with the Alice's pub key:
```bash
target/release/poscan-consensus --base-path /tmp/alice --chain local --alice --port 30333 --ws-port 9944 --rpc-port 9933 --unsafe-rpc-external --node-key 0000000000000000000000000000000000000000000000000000000000000001 --validator -lposcan=debug --author 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
```
Run the second Node:
```bash
target/release/poscan-consensus --base-path /tmp/bob --chain local --bob --port 30334 --ws-port 9945 --rpc-port 9934  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp --validator
```

Copyright (C) 2024 IO https://IOchain.de/
