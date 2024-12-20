# Eye of Sutro: Ethereum State Watcher

<img src="eye_of_sutro.jpg" width="33%" align="right" style="padding-left: 20px"></img>

**Idea**

Ethereum transactions trigger the execution of EVM contract code. The execution is deterministic and depends only on the transaction (sender, calldata, etc) and the chain state (block info, storage). Transactions can make limited changes to chain state and return a bytestring.

We are not interested in accurately computing gas consumption of transactions or transactions involving creating and destroying contracts. This massively simplifies the EVM semantics. We can also safely ignore logging as it is now redundant.

<br style="clear:both;"/>

## Scope

### Order Routing

### Mesh Order Watcher 2.0

### Fast test runner

Ganache is quite slow and this prevents us from running as many tests as we would like. A fast EVM engine that can fork of an existing chain.

## Milestones

<https://eth.wiki/json-rpc/API>

* Forward RPC to Ganache and pass tests.
* Replay a recent block of transactions.
* Replace <https://github.com/0xProject/go-ethereum>
* Pass all tests in <https://github.com/ethereum/tests>
* Run all solutions from <https://g.solidity.cc/>

Debugging:

* Bytes4 decode any call / return value.
* Parse Solidity sourcemaps.

```
clear; RUST_LOG="trace,tokio=info,hyper=info,mio=info" cargo run
```


## Testing using Ethereum tests

```
git clone https://github.com/ethereum/retesteth
cd retesteth
docker build -t retesteth .
cd ..
git clone https://github.com/ethereum/tests
cd tests
docker run --rm -ti -v $(pwd):/tests retesteth -t BlockchainTests/ValidBlocks/VMTests -- --testpath /tests --clients geth --nodes host.docker.internal:8545 --all
```

Run a single test:

```
docker run --rm -ti -v $(pwd):/tests retesteth -t BlockchainTests/ValidBlocks/VMTests/vmBitwiseLogicOperation -- --testpath /tests --clients geth --nodes 192.168.1.2:8545 --singletest xor5 --singlenet Istanbul
```

Stop the tester if it hangs

```
docker kill $(docker ps  | grep retesteth | cut -d ' ' -f 1)
```
