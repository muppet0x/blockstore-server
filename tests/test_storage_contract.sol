pragma solidity >=0.4.22 <0.9.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/StorageContract.sol";

contract TestStorageContract {
    StorageContract storageContract = StorageContract(DeployedAddresses.StorageContract());
    bytes32 testHash = "test_hash_123";

    function testAddFileHash() public {
        bool result = storageContract.addFileHash(testHash);
        Assert.isTrue(result, "Adding file hash should succeed");
    }

    function testRetrieveFileHash() public {
        bytes32 returnedHash = storageContract.getFileHash(0);
        Assert.equal(returnedHash, testHash, "Retrieved hash should match the expected hash");
    }

    function testDuplicateFileHash() public {
        bool result = storageContract.addFileHash(testHash); // Intentionally trying to add the same hash
        Assert.isFalse(result, "Adding duplicate hash should fail");
    }

    function testAddEmptyFileHash() public {
        bytes32 emptyHash = ""; // Solidity automatically treats this as bytes32(0)
        // Directly calling the contract function with low level call to handle return value.
        (bool result, ) = address(storageContract).call(abi.encodeWithSelector(storageContract.addFileHash.selector, emptyHash));
        Assert.isFalse(result, "Adding an empty hash should fail");
    }
}