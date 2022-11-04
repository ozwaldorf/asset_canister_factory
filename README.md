# Asset Canister Factory

This is a simple canister factory that spins up asset canisters on behalf of the caller. The users must be added manually by the initial deployer.

## Usage

Start the local network

```shell
dfx start --clean --background
```

Deploy the canister factory with 64T cycles

```shell
dfx deploy factory --with-cycles 64000000000000
```

Call the `deploy` method to spawn a new canister preloaded with 3T cycles

```shell
dfx canister call factory deploy
```

Call the `upgrade` method to upgrade an existing canister with the currently stored wasm

```shell
dfx canister call factory upgrade '(principal "r7inp-6aaaa-aaaaa-aaabq-cai")'
```

## User Management

The initial deployer can add users to the factory by calling the `add_user` method

```shell
dfx canister call factory add_user '(principal "aaaaa-aa")'
```

Users can be removed by calling the `del_user` method

```shell
dfx canister call factory del_user '(principal "aaaaa-aa")'
```
