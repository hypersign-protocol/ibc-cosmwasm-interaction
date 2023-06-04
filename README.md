# Cross-Chain CosmWasm Contract Interaction

This repo aims to illustrate the cross-chain CosmWasm Smart Contract Interaction. There are two CosmWasm contracts: `controller` and `host`.

The `host` contract maintains a state variable called `count` which is of type `u64`. Its purpose is to increment the value of the `count` variable when it receives an IBC packet from the `controller` contract. Upon successfully incrementing the `count` variable, an acknowledgment is sent back to the `controller` contract, after which, a map named `address_map` is created to store the relationship between the address of the user who initiated the increment request from the `controller` contract and the number of times they have requested to increment the `count` variable in the `host` contract. Essentially, the `controller` contract acts as a reference point for users to initiate cross-chain contract interactions.

Please refer the `contracts/` directory to check the complete implementation of the contracts.

The `.wasm` build files of the contracts are present under the `artifacts/` directory.

## References

This work has been inspired from the following repositories:

- [cw-ibc-example](https://github.com/0xekez/cw-ibc-example)
- [cw-ibc-queries](https://github.com/JakeHartnell/cw-ibc-queries)
- [ics999](https://github.com/larry0x/ics999)
