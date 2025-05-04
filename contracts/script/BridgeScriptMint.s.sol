// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "../src/ZarkToken.sol";

contract BridgeScriptMint is Script {

    uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
    address deployer = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

    function run() external {
        vm.startBroadcast(deployerPrivateKey);
        ZarkToken token = ZarkToken(0x5FbDB2315678afecb367f032d93F642f64180aa3);
        token.mint(deployer, 1000 ether);
        vm.stopBroadcast();
    }
}
// Anvil 8545 : ZarkToken : 0x5FbDB2315678afecb367f032d93F642f64180aa3
// Anvil 8555 : ZarkToken : 0x5FbDB2315678afecb367f032d93F642f64180aa3