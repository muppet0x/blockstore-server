// In StorageContract.sol

function addFileHashes(bytes32[] memory hashes) public returns (bool) {
    for (uint i = 0; i < hashes.length; i++) {
    }
    return true;
}

function getFileHashes(uint[] memory indices) public view returns (bytes32[] memory) {
    bytes32[] memory hashes = new bytes32[](indices.length);
    for (uint i = 0; i < indices.length; i++) {
        hashes[i] = /* logic to retrieve hash by index */;
    }
    return hashes;
}
```

```solidity
// Adjusted example for TestStorageContract using StorageContract enhancements

pragma solidity >=0.4.22 <0.9.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/StorageContract.sol";

contract TestStorageContract {
    StorageContract storageContract = StorageContract(DeployedAddresses.StorageContract());
    bytes32[] testHashes = ["test_hash_123", "test_hash_456"];

    function testBatchAddFileHash() public {
        bool result = storageContract.addFileHashes(testHashes);
        Assert.isTrue(result, "Batch adding file hashes should succeed");
    }

    function testBatchRetrieveFileHash() public {
        uint[] memory indices = new uint[](2);
        indices[0] = 0;
        indices[1] = 1;
        bytes32[] memory returnedHashes = storageContract.getFileHashes(indices);
        for(uint i = 0; i < returnedHashes.length; i++) {
            Assert.equal(returnedHashes[i], testHashes[i], "Each retrieved hash should match the expected hash");
        }
    }
}