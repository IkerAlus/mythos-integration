# Mythos Parachain Integration Guide

## Overview

This is a guide for entities (such as exchanges) that want to integrate with the Mythos Parachain. The Mythos Parachain is meant to be the blockchain platform for Mythos Games.

This guide will guide you through deploying a Mythos Parachain node alongside a Polkadot Relay Chain node, and then launching an instance of Substrate API Sidecar to retrieve information from the chain, along with some examples on how to use it and tools like `subxt` and `Asset Transfer API` to construct and submit transactions.

### Hardware requirements

Currently there are no hardware requirements specific for running a node, since they do not perform time-critical tasks. The only requirement is to have enough storage for the type of node intended, which can be retrieved from [here](https://stakeworld.io/docs/dbsize) for the Polkadot Relay Chain node. Other than that, any relatively performant equipement or any cloud provider will suffice. You can also look into the [reference hardware](https://wiki.polkadot.network/docs/maintain-guides-how-to-validate-polkadot#reference-hardware) for validators, but be aware that these will probably be overkill for a non-validator node.

### Running a Mythos Parachain node

Using the Mythos Parachain will require running a parachain node to sync the chain along with a Polkadot node. Running a Mythos Parachain node is very similar to running a Polkadot node, just with a different binary.

1. Clone the [`Mythos Parachain` repo](https://github.com/paritytech/project-mythical/tree/main) and build from source with the following commands:
  ```bash
  git clone https://github.com/paritytech/project-mythical
  cd project-mythical
  cargo build --release
  ```

  Another alternative is using Docker, this is more advanced, so it's best left up to those already familiar with docker or who have completed the other set-up instructions:

  ```bash
  docker build -t mythos-node --file ./docker/Dockerfile .
  ```

2. Run the node with:

  ```bash
  ./target/release/mythos-node --chain ./chainspecs/mythos-raw.json
  ```
  This will run a Mythos Parachain collator

  You can get more info on the different options for how to run the node with:

  ```bash
  ./target/release/mythos-node --help
  ```

#### How to interact with the node using Substrate API Sidecar

Parity maintains an RPC client, written in TypeScript, that exposes a limited set of endpoints to interact with a node. It handles the metadata and codec logic so that the user is always dealing with decoded information. It also aggregates information that an infrastructure business may need for accounting and auditing, e.g. transaction fees.

You can then attach a Sidecar instance to easily interact with the node by first installing Sidecar:

```bash
npm install @substrate/api-sidecar
# OR
yarn add @substrate/api-sidecar
```

And then running the service within the local directory with:

```bash
export SAS_LOG_WRITE=true # to keep logs in a logs.log file
export SAS_LOG_WRITE_PATH:<path/to/file> # Specifies the path to write the log files. Default will be where the package is installed.
export SAS_SUBSTRATE_URL=<Mythos-Node-RPC-Port> # URL to which the RPC proxy will attempt to connect to
node_modules/.bin/substrate-api-sidecar
```

As an example, for the case of token transfers, Sidecar can fetch information associated with an specific account's balance (http://localhost:8080//accounts/{accountId}/balance-info):

```json
{
  "at": {
    "hash": "string",
    "height": "string"
  },
  "nonce": "string",
  "tokenSymbol": "string",
  "free": "string",
  "reserved": "string",
  "miscFrozen": "string",
  "feeFrozen": "string",
  "frozen": "string",
  "locks": [
    {
      "id": "string",
      "amount": "string",
      "reasons": "Fee = 0"
    }
  ]
}
```

Using the generic endpoints for the `pallet_balances`, Sidecar can also retrieve values specific to the configuration of the pallet, such as `constants`, `dispatchables`, `errors`, `events` and `storage`.

Sidecar can also submit transactions to the node it's connected to. For this we would have to first build the transaction (e.g. using `subxt`), sign it, and submit it as a hex string. Then, if the submission was successful, we would receive a JSON with the hash of the transaction, `txHash`:

```json
{
  "hash": "txHash"
}
```

*Note that the transaction hash is [NOT a unique identifier in the Polkadot ecosystem](https://wiki.polkadot.network/docs/build-protocol-info#unique-identifiers-for-extrinsics).

You can find more information about Sidecar in its [documentation](https://paritytech.github.io/substrate-api-sidecar/dist/).

## MYTH Token transfers

Mythos Parachain's native token is **MYTH**, handled by the [`pallet_balances`](https://docs.rs/pallet-balances/latest/pallet_balances/), and is transferred as any other substrate based chain's native token. MYTH Token has 18 decimals and its existential deposit (the amount of tokens an account must holde before being _reaped_) is 0.001 MYTH.

As the Mythos Parachain uses an instance of the `pallet_balances`, its token is handled by the same calls as, for example, DOT, those being:

- `transfer_keep_alive`, which has in place checks to only transfer an amount that leaves at least the Existential Deposit in the sender's account.
- `transfer_allow_death` which does not have the Existential Deposit's checks as the previous call.
- `transfer_all` which transfers the total balance of an account, causing the account to be reaped.

### Transfer Monitoring

#### Monitoring of local MYTH deposits

Currently, MYTH tokens can be sent and received using the calls mentioned previously, and to keep track of the completion of MYTH transfers the service providers need to monitor local transfers and corresponding `balances (Transfer)` event. This event has the fields `from`, `to` and `amount` indicating the origin account, destination account and amount transferred. This event is followed by either `system (ExtrinsicSuccess)` or by `system (ExtrinsicFailed)`, the latter of which in turn has a field `dispatch_error` with the information regarding the reason behind the transfer failure.

#### Monitoring of cross-chain MYTH deposits

Service providers may be interested to provide cross-chain deposits too, this will allow their users to make deposits to their system from other parachains in the Polkadot ecosystem (for example, [Polkadot Asset Hub](https://wiki.polkadot.network/docs/build-integrate-assets)) with a XCM transfer. In these cases, the event emitted when processing the transfer is the `balances(Deposit)` event. This event has the fields `who` and `amount` indicating the destination account and amount transferred.

### Relevant tooling

The Mythos Parachain will come with almost the same tooling suite [provided for the Relay Chain](https://wiki.polkadot.network/docs/build-integration#recommendation), namely [API Sidecar](https://github.com/paritytech/substrate-api-sidecar), [Polkadot-JS](https://wiki.polkadot.network/docs/learn-polkadotjs-index), [subxt](https://github.com/paritytech/subxt), and the [Asset Transfer API](https://github.com/paritytech/asset-transfer-api). If you have a technical question or issue about how to use one of the integration tools, please file a GitHub issue so a developer can help. Recently, a first stable version of [Polkadot-API](https://papi.how/) has been released offering a more flexible and light weight approach for chain interaction. 


#### For transaction construction

Several tools are available to construct transactions for Polkadot Asset Hub. For example, subxt provides libraries in Rust with great flexibility for transaction construction, whereas Asset Transfer API is focused on offering a simplified interface to build token transfers.

##### transaction signature

Note that Mythos parachain uses the same account format as Ethereum, which implies it uses ECDSA as signature scheme unlike other chains in the Polkadot Ecosystem. For example, if using Polkadot-JS for keys management, the keyring type has to be set as `ethereum`, as it can be seen in the examples below.

##### Examples

Examples on how to build transactions for Mythos parachain can be located in their following directories:

- [Polkadot-JS](/polkadot-js-example/)
- [Subxt](/subxt-example/)
- [Asset Transfer API](/asset-transfer-api-example/)

The instructions on how to run each example are located in their respective `README` files.
