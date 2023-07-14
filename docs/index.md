# Quick Start

## Prerequisites

**csx** (Chainsight command-line execution envirionment) depends on several tools. Therefore, developers must have these tools installed and available in advance.

- [rust](https://www.rust-lang.org/tools/install): To generate Canister code to be compiled into Module
- [cargo-make](https://github.com/sagiegurari/cargo-make): To output interface automatically
- [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install): To interact with Internet Computer
- [ic-wasm](https://github.com/dfinity/ic-wasm): To optimize modules and manipulate metadata

You can confirm that it has been installed and is in the path by running the following in a terminal.

```bash
% rustc --version 
rustc 1.67.1 (d5a82bbd2 2023-02-07)
% cargo make --version
cargo-make 0.36.3
% dfx --version
dfx 0.14.0
% ic-wasm --version
ic-wasm 0.3.7
```

## Install CLI

You can choose between the following two methods of installation.

- Download the prebuild CLI binaries
- Build the CLI from source code

Once you have followed either of these steps to install, you can confirm the installation with the following command.

```bash
csx --version
# -> csx x.y.z
```

> **Warning**  
> Currently only macos and linux environments are supported.  
> This is a limitation of the dfx specification on which it depends.

### Download the prebuild CLI binaries

Install and path the binary file according to your terminal from the following

[Releases · horizonx-tech/chainsight-cli](https://github.com/horizonx-tech/chainsight-cli/releases)

### Build the CLI from source code

Clone this repository and run cargo build.

```bash
cd horizonx-tech/chainsight-cli && cargo build --release
```

## Quick Start for your project

A small modification to the Manifest, which is placed when the project is initially generated, is all that is required to create a deployable component.

Let's actually create a project/component with the project name 'initial_project'.  
The 'new' command can be used to create a project with a specified project name.

```bash
# Create Chainsight project
csx new initial_project
```

Then modify the manifest.
Specifically, modify the rpc_url key to match your project.

Once the manifest is corrected to the correct one, the 'build' command can be executed to generate the module and execution commands to run canister.

```bash
# Build project
csx build --path initial_project
```

If the build command succeeds, deploy with the 'deploy' command.  
Remember to have a dfx network in your local area when deploying locally.

```bash
# Deploy project
# NOTE: If you deploy in local, dfx network must be started (ex: 'dfx start')
dfx start
csx deploy --path initial_project/artifacts
```

When 'deploy' is complete, the 'exec' command sends the actual initialization and start of periodic execution instructions to the component.

```bash
# Initialize Components / Start processing
csx exec --path sample
```

# Terminology

## Manifest

Use **Manifest** to actually tell Chainsight what the developer wants to customize.

Specifically, this is a YAML file, and the developer's intent can be reflected by having the values set according to the format specified on the Chainsight side.

Currently, the following types of Manifest are available.

- Component Manifest
- Project Manifest

## Component

A Canister that contains a Module specialized for the data processes managed within the Chainsight Platform.

Chainsight defines several types of canisters specialized for certain applications, allowing users to select the type that best suits their needs and to freely customize items within that type.

The following Components are currently available on the CLI.

- Snapshot Indexer
  - In addition, depending on the data source, you can choose from
    - chain: For EVM-based Other chains
    - canister: Other canisters on Internet Computer
- Relayer

> **Note**  
> See following for all supported Component Types and their purpose.  
> [Data Processing Components - Chainsight Network](https://docs.chainsight.network/chainsight-architecture/data-processing-components)

## Project

Manage one or more Components together.

For example, at least Snapshot Indexer and Relayer are required to take a snapshot of ERC20's totalSupply obtained from Ethereum and flow it to other Chains.

Most objectives require combining multiple Components, and Project is provided to make them easier to manage.

Project consists of the following folders.  
However, these are not usually necessary to be aware of.

```txt
(project root)
|- artifacts # Artifacts generated by 'build' process
|- components # Place Component Manifest
|- interfaces # Place dependent interfaces (abi etc.)
|- project.yaml # Project Manifest
.chainsight
```

---

These are the Internet Computer concepts needed to better understand Chainsight

## HTTPS outcalls

HTTPS outcalls in the Internet Computer refer to making HTTP requests from a canister (smart contract) running on the Internet Computer to external HTTPS endpoints. It allows canisters to interact with external services, such as APIs or web servers, over secure HTTPS connections.

HTTPS outcalls are subject to certain limitations and security considerations imposed by the Internet Computer platform. For example, canisters have a set of allowed domains and endpoints they can make requests to, and the response size and request duration may be limited. These restrictions are in place to ensure the security, scalability, and resource efficiency of the Internet Computer network.

For more information, please check here.

[HTTPS outcalls: technology overview | Internet Computer](https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-how-it-works)

## Timer Task

Unlike other blockchains, the Internet Computer can automatically execute canister smart contracts after a specified delay or periodically.

For more information, please check here.

[Periodic tasks and timers | Internet Computer](https://internetcomputer.org/docs/current/developer-docs/backend/periodic-tasks)

# Commands

Several subcommands of csx can be used to advance the development of the component.

The basic syntax is as follows

```bash
csx [subcommand] [flag]
```

You can also check the list of executable commands by specifying the `--help` flag.

## csx new

Create a new project for Chainsight Platform.

This generated project contains several Manifests as templates.  
Specify the name of the project to be created, and then generate the project.

```bash
csx new sample_project
```

## csx create

The 'create' command is used to add a new type of component to your project.

This command will add a Component Manifest of the specified Type and its management settings to the Project Manifest.

If you are familiar with it, you can do manually what this command does.

```bash
% csx create --help
Generates component manifest of specified type and adds to your project

Usage: csx create [OPTIONS] --type <TYPE> <COMPONENT_NAME>

Arguments:
  <COMPONENT_NAME>
          Specifies the name of the component to create

Options:
      --type <TYPE>
          Specifies type of the component to create

          Possible values:
          - event-indexer:     To synchronize event data
          - algorithm-indexer: To store data from chainsight-canisters and collect them into other types of data
          - snapshot:          To periodically take and store snapshots from Contract and other Canisters
          - relayer:           To relay data to other blockchains

  -v, --verbose...
          Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance

      --path <PATH>
          Specify the path of the project to which the component is to be added. If not specified, the current directory is targeted

  -q, --quiet...
          Suppresses informational messages. -qq limits to errors only; -qqqq disables them all

  -h, --help
          Print help (see a summary with '-h')
```

- `--type`: Specify the Component Type
  - A Template Manifest will be generated for the specified Component Type.
- `--path`: Select the path of the project to which you want to add the Component.
  - The folder containing the `.chainsight` file will be recognized as the project.

## csx build

Performs everything from code generation to compilation of the Module to deploy from Manifest to Canister.

You must specify the project path to build and execute this command.

```bash
csx build --path sample_project
```

> **Note** For developers who want more control  
> (The specifications are still under review and subject to change.) The command is divided into two phases: codegen and build. The developer can pass a flag to execute only one of them.  
> `--only-codegen`: Perform code generation only  
> `--only-build`: Perform build only (from already generated code)`

```bash
% csx build --help 
Builds your project to generate canisters' modules for Chainsight

Usage: csx build [OPTIONS]

Options:
      --path <PATH>   Specify the path of the project to be built. If not specified, the current directory is targeted
  -v, --verbose...    Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance
      --only-codegen  Only perform code generation
  -q, --quiet...      Suppresses informational messages. -qq limits to errors only; -qqqq disables them all
      --only-build    Only perform build. Perform this steps with code already generated
  -h, --help          Print help
```

## csx deploy

This command is used to deploy a built module of your own Project to a specified network (local or IC).

It is built by wrapping the operations performed by the dfx deploy command, plus Identity and Wallet checks.

- `--path`: Specify the path where the built artifacts are located.
  - Build artifacts are located in `/artifacts` under the project folder.
  - ex: If the project folder is `./sample_project`, the build artifacts will be located in `./sample_project/artifacts`.
    - In that case, the command is `dfx deploy -path sample_project/artifacts`.
- `--network`: Specify the network to deploy to.
  - Currently, you can choose between the following options.
    - local ... localhost
    - ic ... mainnet of Internet Computer

```bash
% csx deploy --help
Deploy the components of your project. If you want to operate on a local network, you need to build a local dfx network in advance

Usage: csx deploy [OPTIONS]

Options:
      --path <PATH>        Specify the path of the project to be deployed. If not specified, the current directory is targeted
  -v, --verbose...         Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance
      --network <NETWORK>  Specify the network to execute on [default: local] [possible values: local, ic]
  -q, --quiet...           Suppresses informational messages. -qq limits to errors only; -qqqq disables them all
      --port <PORT>        Specifies the port to call. This option is used only if the target is localhost
  -h, --help               Print help
```

## csx exec

Executes commands that prepare the deployed project's Components for correct operation. The commands executed here are built from the information in the Component Manifest.

It is currently implemented to perform the following commands.

1. Setup: Set parameters for the Component to operate as intended.
2. Start timer task: kick periodic execution of data acquisition/processing/storage.

```bash
% csx exec --help  
Calls for component processing. Currently supports initialization and task start instructions

Usage: csx exec [OPTIONS]

Options:
      --path <PATH>            Specify the path of the project that manages the component to be called. Refer to the manifest of this project to build the commands that should be executed
  -v, --verbose...             Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance
      --component <COMPONENT>  Specify the name of the component you want to execute. If this option is not specified, the command will be given to all components managed by the project
  -q, --quiet...               Suppresses informational messages. -qq limits to errors only; -qqqq disables them all
      --network <NETWORK>      Specify the network to execute on [default: local] [possible values: local, ic]
      --only-generate-cmds     Only generate commands
      --only-execute-cmds      Only execute commands. Perform this steps with commands already generated
  -h, --help                   Print help
```

## csx remove

Used to delete a project that has already been created.

> **Warning**  
> Stopping a deployed Component (Canister) is currently not part of the process, so you will need to stop/delete the Component, canister, manually.

```bash
dfx remove --path sample_project
```

# How to customize

## About Manifest

The developer must update the manifest to meet his/her own objectives and communicate his/her intentions to the component.

This section describes how to define and modify Manifest.

### Project Manifest

This section describes the manifest of the component managed by your project.

The version is currently fixed at "v1" only.

example)

```yaml
version: v1
label: example_pj
components:
- component_path: components/example_event_indexer.yaml
- component_path: components/example_algorithm_indexer.yaml
- component_path: components/example_pj_snapshot_chain.yaml
- component_path: components/example_pj_snapshot_icp.yaml
- component_path: components/example_pj_relayer.yaml
```

### Component Manifest

The description of each component type is different, but the following is a description of the common parts.

`version`: Like the project manifest, currently fixed at “v1”.

`metadata`: These are the places where the meta-information of the Component, which can be any Type, is stored, and are set in the custom attributes of WASM according to the specification of Internet Computer.

Of particular importance is the type, which determines the Component Type, so be sure to check and set it.

- `label`: String / Component name
- `type`: Enum / Component Type (ex: snapshot)
- `description`: String / Component description field

example)

```yaml
version: v1
metadata:
  label: example_pj_snapshot_chain
  type: snapshot
  description: ''
...
```

#### Snapshot

As mentioned earlier, there are multiple types of Snapshot, and each datasource has different logic and external connection methods, so there are differences in the way manifest is written.

The Snapshot type is selected in `datasource.type`.

**Snapshot (datasource = contract)**

Select this type if you want to build a data snapshot using HTTPS outcalls from an EVM-compliant blockchain.

In `datasource`, specify the destination network, contract address, and contract function.

- `datasource.type`: fixed at “contract”.
- `datasource.location` ... Specify destination network, contract
  - `id`: String / Contract address
  - `args.network_id`: Number / chain_id of the target network
  - `args.rpc_url`: String / rpc endpoint url to connect to the target network
- `datasource.method` ... Specify the function of the contract to be called.
  - `method.identifier`: String / Interface of the function to be called.
    - Please refer to the interfaces mentioned in the repository below.
      - [rust-ethereum/ethabi: Encode and decode smart contract invocations](https://github.com/rust-ethereum/ethabi)
  - `method.interface`: Name of the ABI file containing the interface to the above function.
    - Placed under `(project folder)/interfaces`
    - ERC20.json is buildin to the CLI.
  - `method.args`: String, Number / If the function has arguments, set their values.
    - The value entered is always used in HTTPS outcalls as a fixed value.
      - Snapshot stores function calls with the same conditions as a Snapshot, so it is not possible to calculate the input value each time.

`storage` selects how to store the retrieved values as specified in `datasource`

- `storage.with_timestamp`: boolean / Determines whether to include the timestamp when saving.
  - If this is true, the timestamp obtained using Internet Computer's API is included in the Snapshot data and stored in the historical data.

`interval` specifies the interval between acquiring and storing data at the destination specified by `datasource`.

- `interval`: Number / Interval between data acquisition and storage.
  - Set in seconds.

example)

```yaml
version: v1
metadata:
  label: example_pj_snapshot_chain
  type: snapshot
  description: ''
datasource:
  type: contract
  location:
    id: 6b175474e89094c44da98b954eedeac495271d0f
    args:
      network_id: 1
      rpc_url: https://eth-mainnet.g.alchemy.com/v2/<YOUR_KEY>
  method:
    identifier: totalSupply():(uint256)
    interface: ERC20.json
    args: []
storage:
  with_timestamp: true
interval: 3600
```

**Snapshot (datasource = canister)**

Select this type if you want to use cross canister calls within Internet Computer to build a snapshot of data that can be retrieved from other canisters.

Since `datasource` differs from contract only in the `datasource` field, only `datasource` is described in this section.

Set the function for the canister to be acquired at `datasource`.

- `datasource.location` ... Specify the canister to retrieve from.
  - `id`: String / Target canister
    - The input format differs according to `args.id_type`
      - If id_type = `canister_name`, Name of canister in the project.
      - If id_type = `principal_id`, Text Format of the target canister's Principal.
  - `args.id_type`: Enum / How to set `id`
- `datasource.method` ... Specify the Canister function to call.
  - `method.identifier`: String / interface of the function to be called.
    - Refer to the interface definition in the candid file and enter.
      - NOTE: Exclude "query" in the query call.
  - `method.interface`: Current status "null" fixed.
  - `method.args`: String, Number / If the function has arguments, set their values
    - The value entered is always used in cross contract calls as a fixed value.

example)

```yaml
version: v1
metadata:
  label: example_pj_snapshot_icp
  type: snapshot
  description: ''
datasource:
  type: canister
  location:
    id: sample_pj_snapshot_chain
    args:
      id_type: canister_name
  method:
    identifier: 'get_last_snapshot : () -> (record { value : text; timestamp : nat64 })'
    interface: null
    args: []
storage:
  with_timestamp: true
interval: 3600
```

#### Relayer

Relayer is a component for propagating data computed and maintained by Chainsight Platform to other blockchains.

Therefore, `datasource` and `interval` are the same as Snapshot for Canister, and a new `destination` must be specified to specify the propagation destination.

- `destination.network_id`: Number / chain_id of the target network
- `destination.type`: Enum / Oracle Type
  - To send data to the destination chain, Chainsight provides several Oracle types according to a defined standard, and the developer specifies the type
- `destination.oracle_address`: String / Oracle address
- `destination.rpc_url`: String / rpc endpoint url to call the target network

example)

```yaml
version: v1
metadata:
  label: example_pj_relayer
  type: relayer
  description: ''
datasource:
  type: canister
  location:
    id: sample_pj_snapshot_chain
    args:
      id_type: canister_name
  method:
    identifier: 'get_last_snapshot_value : () -> (text)'
    interface: null
    args: []
destination:
  network_id: 80001
  type: uint256
  oracle_address: 0539a0EF8e5E60891fFf0958A059E049e43020d9
  rpc_url: https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>
interval: 3600
```

**What is Oracle?**

When Relayer propagates data to other blockchains, Chainsight provides Oracle Contract as the target.

It has only a state that stores the specified type in key/value format, where the key is an address constructed from the sender's Canister secret information, allowing the sender to be identified.

The following is a tentative specification, but there is a specific code below.

[horizonx-tech/chainsight-evm-oracles](https://github.com/horizonx-tech/chainsight-evm-oracles)

Currently, only EVM compatible chains are supported, so only Solidity files are placed.