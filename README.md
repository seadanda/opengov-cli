# OpenGov CLI

This program's primary purpose is to construct all the needed calls to submit a proposal as an OpenGov referendum on Kusama or Polkadot. Governance lives on Asset Hub, so the generated calls target Kusama Asset Hub or Polkadot Asset Hub. It assumes that you construct the proposal (i.e., the privileged call you want to execute on Asset Hub) elsewhere (e.g. the [PAPI Extrinsics UI](https://dev.papi.how/extrinsics)). It will return all the calls that you will need to sign and submit. Note that you may need to submit calls on multiple chains (e.g. Asset Hub and Collectives).

It also provides a utility to construct a runtime upgrade call that will batch the upgrades of all system chains in a network. The batch is constructed as a call on Asset Hub that uses XCM to send `authorize_upgrade` to each parachain and the Relay Chain.

## CLI

This is a CLI program. To get started:

```
$ git clone https://github.com/joepetrowski/opengov-cli.git
$ cd opengov-cli
$ cargo build
$ ./target/debug/opengov-cli --help
Utilities for submitting OpenGov referenda and constructing tedious calls

Usage: opengov-cli <COMMAND>

Commands:
  build-upgrade      Generate a single call that will upgrade all system chains in a given network
  submit-referendum  Generate all the calls needed to submit a proposal as a referendum in OpenGov
  help               Print this message or the help of the given subcommand(s)

Options:
  -V, --version  Print version
  -h, --help     Print help
```

### Submit Referendum

The `submit-referendum` subcommand will take a proposal and some parameters and create all the necessary calls. Note that they can actually be submitted in any order. The preimages do not need to be submitted in order to start the referenda, but they will eventually in order to enact.

```
$ ./target/debug/opengov-cli submit-referendum --help
Generate all the calls needed to submit a proposal as a referendum in OpenGov

Usage: opengov-cli submit-referendum [OPTIONS] --proposal <PROPOSAL> --network <NETWORK> --track <TRACK>

Options:
  -p, --proposal <PROPOSAL>
          The encoded proposal that we want to submit. This can either be the call data itself, e.g. "0x0102...", or a file path that contains the data, e.g. "./my_proposal.call"
  -n, --network <NETWORK>
          Network on which to submit the referendum. `polkadot` or `kusama`
  -t, --track <TRACK>
          Track on which to submit the referendum
      --at <AT>
          Optional: Enact at a particular block number
      --after <AFTER>
          Optional: Enact after a given number of blocks
      --fellowship <FELLOWSHIP>
          Optional: Use the Fellowship on another network for whitelisting (`polkadot` or `kusama`)
      --output-len-limit <OUTPUT_LEN_LIMIT>
          Output length limit. Defaults to 1,000
      --no-batch
          Do not print batch calls. Defaults to false
      --output <OUTPUT>
          Form of output. `AppsUiLink` or `CallData`. Defaults to Apps UI
      --light-client
          Use light client endpoints instead of RPC for PAPI links
  -h, --help
          Print help
```

### Build Upgrade

The `build-upgrade` subcommand will take a network name and Fellowship release version and construct a single call to upgrade all system chains. The output is a `.call` file that can be passed directly to `submit-referendum`.

```
$ ./target/debug/opengov-cli build-upgrade --help
Generate a single call that will upgrade all system chains in a given network

Usage: opengov-cli build-upgrade [OPTIONS] --network <NETWORK>

Options:
  -n, --network <NETWORK>              Network on which to submit the referendum. `polkadot` or `kusama`
      --only                           Only include the runtimes explicitly specified
      --local                          Use local WASM files instead of downloading from GitHub
      --relay-version <RELAY_VERSION>  The Fellowship release version. Should be semver and correspond to the release published
      --asset-hub <ASSET_HUB>          Optional. The runtime version of Asset Hub to which to upgrade
      --bridge-hub <BRIDGE_HUB>        Optional. The runtime version of Bridge Hub to which to upgrade
      --collectives <COLLECTIVES>      Optional. The runtime version of Collectives to which to upgrade
      --encointer <ENCOINTER>          Optional. The runtime version of Encointer to which to upgrade (Kusama only)
      --people <PEOPLE>                Optional. The runtime version of People to which to upgrade
      --coretime <CORETIME>            Optional. The runtime version of Coretime to which to upgrade
      --filename <FILENAME>            Name of the file to which to write the output
      --additional <ADDITIONAL>        Some additional call that you want executed on Asset Hub along with the upgrade
  -h, --help                           Print help
```

## Examples

### Build Upgrade

```
$ ./target/release/opengov-cli build-upgrade --network polkadot --relay-version "1.0.0"

Downloading runtimes.

Downloading... polkadot_runtime-v1000000.compact.compressed.wasm
Downloading... asset-hub-polkadot_runtime-v1000000.compact.compressed.wasm
Downloading... collectives-polkadot_runtime-v1000000.compact.compressed.wasm
Downloading... bridge-hub-polkadot_runtime-v1000000.compact.compressed.wasm
Downloading... people-polkadot_runtime-v1000000.compact.compressed.wasm
Downloading... coretime-polkadot_runtime-v1000000.compact.compressed.wasm

Generating parachain authorization calls. The runtime hashes are logged if you
would like to verify them with srtool.

Polkadot Relay Chain Runtime Hash: 0x17e1c7023134c196678f202daf8071a25f63f3e7b3937d8632a7474c618dc9a4
Polkadot Asset Hub Runtime Hash:   0x52c2f520914514a196059fc8cc74f516a004f2463ba11d7385b5241bb5d50ee4
Polkadot Collectives Runtime Hash: 0xb061815642328374a62b3282c78fa8bef5a27cd313d4ac79cbd49e43e0a4b879
Polkadot Bridge Hub Runtime Hash:  0xf43e890a5eca0230a7eaaf88a60b8cc5ccb0ef157986f628650b071e47f7d323

Batching calls.

Success! The call data was written to ./upgrade-polkadot-1.0.0/polkadot-1.0.0.call
To submit this as a referendum in OpenGov, run:

opengov-cli submit-referendum \
    --proposal "./upgrade-polkadot-1.0.0/polkadot-1.0.0.call" \
    --network "polkadot" --track <"root" or "whitelistedcaller">
```

### Submit a Referendum on Kusama

> **Note:** The example output below was generated before the Asset Hub migration and shows `networkId=kusama` (Relay Chain) in the PAPI links. Current output will show `networkId=kusama_asset_hub` for the public referendum calls and `networkId=kusama` for the Fellowship calls (which target the Relay Chain on Kusama).

As a proposal, send an XCM to authorize an upgrade on Kusama Asset Hub.

Call data:
```
0x630001000100a10f0204060202286bee880102957f0c9b47bc84d11116aef273e61565cf893801e7db0223aeea112e53922a4a
```

This has a call hash of `0x4149bf15976cd3c0c244ca0cd43d59fed76f4bb936b186cc18bd88dee6edd986`.

```
$ ./target/debug/opengov-cli submit-referendum \
	--proposal "0x630001000100a10f0204060202286bee880102957f0c9b47bc84d11116aef273e61565cf893801e7db0223aeea112e53922a4a" \
	--network "kusama" --track "whitelistedcaller" \
	--after "10"

Submit the preimage for the Fellowship referendum:
https://dev.papi.how/extrinsics#data=0x2000882c004149bf15976cd3c0c244ca0cd43d59fed76f4bb936b186cc18bd88dee6edd986&networkId=kusama&endpoint=wss%3A%2F%2Fkusama-rpc.dwellir.com

Open a Fellowship referendum to whitelist the call:
https://dev.papi.how/extrinsics#data=0x17002b0f024c02d09f7b5e4b71e357780baf8cb2d625dca6efaba2ee777516eaf72e5a14a022000000010a000000&networkId=kusama&endpoint=wss%3A%2F%2Fkusama-rpc.dwellir.com

Submit the preimage for the public referendum:
https://dev.papi.how/extrinsics#data=0x2000d42c03630001000100a10f0204060202286bee880102957f0c9b47bc84d11116aef273e61565cf893801e7db0223aeea112e53922a4a&networkId=kusama_asset_hub&endpoint=wss%3A%2F%2Fasset-hub-kusama-rpc.dwellir.com

Open a public referendum to dispatch the call:
https://dev.papi.how/extrinsics#data=0x15002b0d02022022c662d88f6b0f84c1771134e69b4412aff9e08a99e2a2da2794b5725fbe35000000010a000000&networkId=kusama_asset_hub&endpoint=wss%3A%2F%2Fasset-hub-kusama-rpc.dwellir.com

Batch to submit on Kusama Asset Hub:
https://dev.papi.how/extrinsics#data=0x1804102000882c004149bf15976cd3c0c244ca0cd43d59fed76f4bb936b186cc18bd88dee6edd98617002b0f024c02d09f7b5e4b71e357780baf8cb2d625dca6efaba2ee777516eaf72e5a14a022000000010a0000002000d42c03630001000100a10f0204060202286bee880102957f0c9b47bc84d11116aef273e61565cf893801e7db0223aeea112e53922a4a15002b0d02022022c662d88f6b0f84c1771134e69b4412aff9e08a99e2a2da2794b5725fbe35000000010a000000&networkId=kusama_asset_hub&endpoint=wss%3A%2F%2Fasset-hub-kusama-rpc.dwellir.com
```

This will return either two or four calls, the latter if the origin is `WhitelistedCaller`, which will require a preimage and referendum for the Fellowship. It also returns a batch call if you want to submit them all at once (you can hide this with `--no-batch`).

### Submit a Referendum on Polkadot

For Polkadot, we will use a proposal of `0x0000645468652046656c6c6f777368697020736179732068656c6c6f`, which is a `system.remark` call. We will use the Fellowship to whitelist it.

The Fellowship is on the Collectives parachain, so this will require a referendum on the Collectives chain for the Fellowship to whitelist a call, and a referendum on Asset Hub for it to pass public vote. Notice the PAPI links pointing to different chains in the output.

> **Note:** The example call data and hashes below are from a pre-AHM run. Current output will show `networkId=polkadot_asset_hub` for the public referendum calls.

```
$ ./target/debug/opengov-cli submit-referendum \
	--proposal "0x0000645468652046656c6c6f777368697020736179732068656c6c6f" \
	--network "polkadot" --track "whitelistedcaller" \
	--after "10"

Submit the preimage for the Fellowship referendum:
https://dev.papi.how/extrinsics#data=0x2b00d41f0003010003082f00000603c2695e6d216f8817000363631d09c4ac33f2960d5d26b02f8ec89ac7a986c0bdab2a3a9f354acb6167&networkId=polkadot_collectives&endpoint=wss%3A%2F%2Fpolkadot-collectives-rpc.polkadot.io

Open a Fellowship referendum to whitelist the call:
https://dev.papi.how/extrinsics#data=0x3d003e0102664da7c8fb74a75e641b8aca751297fff57c5aee8014b3570e08f1454c06a88b35000000010a000000&networkId=polkadot_collectives&endpoint=wss%3A%2F%2Fpolkadot-collectives-rpc.polkadot.io

Submit the preimage for the public referendum:
https://dev.papi.how/extrinsics#data=0x0a007817030000645468652046656c6c6f777368697020736179732068656c6c6f&networkId=polkadot_asset_hub&endpoint=wss%3A%2F%2Fasset-hub-polkadot-rpc.dwellir.com

Open a public referendum to dispatch the call:
https://dev.papi.how/extrinsics#data=0x1500160d022d1d8846a18770fc07a5b03383045d965aad65abb1077d0306142e60551813141e000000010a000000&networkId=polkadot_asset_hub&endpoint=wss%3A%2F%2Fasset-hub-polkadot-rpc.dwellir.com

Batch to submit on Polkadot Asset Hub:
https://dev.papi.how/extrinsics#data=0x1a04080a007817030000645468652046656c6c6f777368697020736179732068656c6c6f1500160d022d1d8846a18770fc07a5b03383045d965aad65abb1077d0306142e60551813141e000000010a000000&networkId=polkadot_asset_hub&endpoint=wss%3A%2F%2Fasset-hub-polkadot-rpc.dwellir.com

Batch to submit on Polkadot Collectives Chain:
https://dev.papi.how/extrinsics#data=0x2804082b00d41f0003010003082f00000603c2695e6d216f8817000363631d09c4ac33f2960d5d26b02f8ec89ac7a986c0bdab2a3a9f354acb61673d003e0102664da7c8fb74a75e641b8aca751297fff57c5aee8014b3570e08f1454c06a88b35000000010a000000&networkId=polkadot_collectives&endpoint=wss%3A%2F%2Fpolkadot-collectives-rpc.polkadot.io
```

### Checking the Results

The output consists of [PAPI Extrinsics](https://dev.papi.how/extrinsics) links. Open each link in a browser to inspect the decoded call before signing.

For whitelisted-caller referenda, verify:

1. The **Fellowship preimage** contains a `whitelist.whitelist_call` with the hash of your original proposal
2. The **Fellowship referendum** is submitted with the `Fellows` origin and references the whitelist preimage — note the hash is for the `whitelist_call` instruction, _not_ your actual proposal
3. The **public preimage** contains your original proposal wrapped in `whitelist.dispatch_whitelisted_call_with_preimage`
4. The **public referendum** is submitted on the `whitelisted-caller` track — the hash references the _dispatch_ instructions that contain your actual proposal

For non-whitelisted referenda (root, staking-admin, etc.), verify:

1. The **preimage** contains your original proposal in a `preimage.note_preimage` call
2. The **referendum** is submitted on the correct track and references the preimage hash

Use `--output calldata` if you want raw call data for verification in other tools.
