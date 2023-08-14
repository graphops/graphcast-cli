# One shot CLI

[![Docs](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.graphops.xyz/graphcast/radios/one-shot)
[![crates.io](https://img.shields.io/crates/v/one-shot-cli.svg)](https://crates.io/crates/one-shot-cli)

## Introduction

This is a Graphcast Radio focused on sending a single message about particular subgraphs on Graphcast P2P network. The available message type is `UpgradeIntentMessage` from a subgraph owner.

### Upgrade Pre-sync

When developers publish a new version (subgraph deployment) to their subgraph, data service instability may occur while their API queries the pre-existing version. Indexers may require some time to sync a subgraph to the chainhead after they have stopped syncing the previous deployment. To decrease the upgrade friction, developers can send a message before publishing the subgraph on current deployment hash's Graphcast channel, which includes the subgraph id, corresponding new deployment hash, and the represented graph account that must be validated to be the subgraph's owner. 

Indexers running the subgraph radio and listening to that channel will in turn receive the message and potentially start to offchain new deployment.

It is still at the subgraph developers' discretion to await for the indexers to sync upto chainhead, in which point they can publish the staged version without disrupting API usage.

## 🆙 Example usage

To send a message on Graphcast mainnet for the subgraph deployment "QmacQnSgia4iDPWHpeY6aWxesRFdb8o5DKZUx96zZqEWrB", we would need its subgraph id "CnJMdCkW3pr619gsJVtUPAWxspALPdCMw6o7obzYBNp3", private key to the graph account or an operator of the graph account, the subgraph owner's graph account, and the new deployment hash. You can supply them as an CLI argument

```
cargo run -- --private-key "abcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefg1" \
--graph-account "0xe9a1cabd57700b17945fd81feefba82340d9568f" \
--new-hash "QmVVfLWowm1xkqc41vcygKNwFUvpsDSMbHdHghxmDVmH9x" \
--subgraph-id "CnJMdCkW3pr619gsJVtUPAWxspALPdCMw6o7obzYBNp3"
```

The entire process from running the binary to sending the message should take ~45 seconds. One can expect the terminal to output the following:

```
2023-07-31T17:56:44.783866Z  INFO Creating Graphcast Agent, radio_name: "subgraph-radio", registry_subgraph: "https://api.thegraph.com/subgraphs/name/hopeyen/graphcast-registry-goerli", network_subgraph: "https://api.thegraph.com/subgraphs/name/graphprotocol/graph-network-goerli", graphcast_network: "testnet", max_retry: 5

2023-07-31T17:56:59.328490Z  INFO Sent message, msg_id: "0xc6b1131e0f8302abe48057f6fc69722ab46bd4285c2c4a8a8bdca6b221dcda96"
```


## 🧪 Testing

To run unit tests for the Radio. We recommend using [nextest](https://nexte.st/) as your test runner. Once you have it installed you can run the tests using the following commands:

```
cargo nextest run
```

## Contributing

We welcome and appreciate your contributions! Please see the [Contributor Guide](/CONTRIBUTING.md), [Code Of Conduct](/CODE_OF_CONDUCT.md) and [Security Notes](/SECURITY.md) for this repository.
