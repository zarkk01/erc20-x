// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "../src/ZarkToken.sol";

contract BridgeScriptDeploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

        vm.startBroadcast(deployerPrivateKey);

        ZarkToken token = ZarkToken(0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512);

        token.bridge(100 ether, deployer);

        vm.stopBroadcast();
    }
}