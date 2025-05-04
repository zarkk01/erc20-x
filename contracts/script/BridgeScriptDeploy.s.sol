// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "../src/ZarkToken.sol";

contract BridgeScriptDeploy is Script {
    ZarkToken public zarkToken;
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        zarkToken = new ZarkToken("ZARK", "ZRK", address(0)); // no relayer for now
        
        vm.stopBroadcast();
    }
}