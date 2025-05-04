/// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ZarkToken
 * @dev ERC20 token with burn-and-mint bridge functionality.
 */
contract ZarkToken is ERC20, ERC20Burnable, Ownable {
    // Address permitted to call mintFromBridge
    address public relayer;

    // Record processed bridge transactions to prevent replays
    mapping(bytes32 => bool) public processed;

    event BridgeInitiated(
        address indexed from,
        address indexed to,
        uint256 amount,
        uint256 indexed srcChainId,
        bytes32 txHash
    );

    modifier onlyRelayer() {
        require(msg.sender == relayer);
        _;
    }

    constructor(
        string memory name_,
        string memory symbol_,
        address relayer_
    ) ERC20(name_, symbol_) Ownable(msg.sender) {
        relayer = relayer_;
    }

    /**
     * @dev Burns tokens on this chain and emits event for off-chain relayer to pick up.
     * @param amount Number of tokens to bridge.
     * @param to Recipient address on destination chain.
     */
    function bridge(uint256 amount, address to) external {
        _burn(msg.sender, amount);

        // Include tx origin data to uniquely identify the bridging request
        bytes32 txHash = keccak256(
            abi.encodePacked(msg.sender, to, amount, block.chainid, block.number)
        );

        emit BridgeInitiated(msg.sender, to, amount, block.chainid, txHash);
    }

    /**
     * @dev Mints tokens on this chain when a valid bridge event is observed on source chain.
     * Can only be called by the designated relayer.
     * @param to Recipient address on this chain.
     * @param amount Number of tokens to mint.
     * @param srcChainId Source chain ID (for reference).
     * @param txHash Unique identifier of the bridge transaction.
     */
    function mintFromBridge(
        address to,
        uint256 amount,
        uint256 srcChainId,
        bytes32 txHash
    ) external onlyRelayer {
        require(!processed[txHash], "Bridge: transaction already processed");
        processed[txHash] = true;
        _mint(to, amount);
    }

    /**
     * @dev Allows owner to update the relayer address.
     */
    function updateRelayer(address newRelayer) external onlyOwner {
        relayer = newRelayer;
    }
}
