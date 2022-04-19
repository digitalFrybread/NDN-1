<div align="center">

  <h1><code>SMARTRIB3-NDN</code></h1>

  <strong>A Native Derivatives Network is a  <a href="https://github.com/paritytech/substrate">Substrate</a> based blockchain that tribal communities 
  can use as bridge to explore and get hands on with next generation technology </strong>

  <h3>
    <a href="https://substrate.io/">Docs</a>
    <span> | </span>
    <a href="https://matrix.to/#/!HzySYSaIhtyWrwiwEV:matrix.org?via=matrix.parity.io&via=matrix.org&via=web3.foundation">Chat</a>
  </h3>

</div>

## Features

This template includes the minimum required components to start a PoS testnet, inspired by [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template).

* Consensus related pallets: Babe & GRANDPA
* Staking related pallets: staking, session, authorship, im-online, offences, utility
* Governance related pallets: collective, membership, elections-phragmen, democracy, treasure

**Notes:** The code is un-audited and not production ready, use it at your own risk.

## Getting Started

Follow the steps below to get started.

### Rust Setup

First, complete the [Dev Docs Installation](https://docs.substrate.io/v3/getting-started/installation/).

### Build and Run

Use the following command to build the node and run it after build successfully:

```sh
cargo build --release
./target/release/substrate-ndn --dev
```

## Run public testnet

* Modify the genesis config in chain_spec.rs
* Build spec, `./target/release/substrate-ndn build-spec --chain staging > ndn-staging.json`
* Change original spec to encoded raw spec, `./target/release/substrate-ndn build-spec --chain=ndn-staging.json --raw > ndn-staging-raw.json`
* Start your bootnodes, node key can be generate with command `./target/release/substrate-ndn key generate-node-key`.
  ```shell
  ./target/release/substrate-ndn \
       --node-key 0xe33b82d1da14ba60ef6c7e97857ef2c0552d26d79452f686fcc670ce754059be \
       --base-path /tmp/bootnode1 \
       --chain ndn-staging-raw.json \
       --name NDNode
  ```
* Start your initial validators,
  ```shell
  ./target/release/substrate-ndn \
      --base-path  /tmp/validator1 \
      --chain   ndn-staging-raw.json \
	    --port 30336 \
	    --ws-port 9947 \
	    --rpc-port 9936 \
      --name  validator1 \
      --validator
  ```
* [Insert session keys](https://substrate.dev/docs/en/tutorials/start-a-private-network/customchain#add-keys-to-keystore)
* Attract enough validators from community in waiting
* Call force_new_era in staking pallet with sudo, rotate to PoS validators
* Enable governance, and remove sudo
* Enable transfer and other functions
