// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "../src/ZarkToken.sol";

contract BridgeScriptDeploy is Script {

    uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
    ZarkToken public zarkToken;

    function run() external {
        vm.startBroadcast(deployerPrivateKey);
        zarkToken = new ZarkToken("ZARK", "ZRK", 0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f);
        vm.stopBroadcast();
    }
}
// Anvil 8545 : ZarkToken : 0x5FbDB2315678afecb367f032d93F642f64180aa3
// Anvil 8555 : ZarkToken : 0x5FbDB2315678afecb367f032d93F642f64180aa3