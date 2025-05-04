// common pattern on block rust programs to have a bindings.rs / contracts.rs / abi.rs file in order to have in file all the abis of the smart contracts
// importing all essential parts from ethers-rs lib and get Provider, Signer, Contract, Address etc and all building blocks to interract with ethereum.
use ethers::prelude::*;

abigen!(
    ZarkToken, // this would be the name of the struct that will essentially represent the whole contract in the rust script (it will use it like `let contract = ZarkToken::new(address, client);`)
    "../contracts/out/ZarkToken.sol/ZarkToken.json" // with this way we take the abi and the bytecode of the ZarkToken. (maybe there would be a better way to do that, so to not make the script here dependable of the `contracts` folder)
);
