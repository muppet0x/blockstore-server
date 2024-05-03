pragma solidity >=0.4.22 <0.9.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/StorageContract.sol";

contract TestStorageContract {
    StorageContract storageContract = StorageContract(DeployedAddresses.StorageContract());

    function testAddFileHash() public {
        bytes32 testHash = "test_hash_123";
        bool result = storageContract.addFileHash(testHash);
        Assert.isTrue(result, "Adding file hash should succeed");
    }

    function testRetrieveFileHash() public {
        bytes32 expectedHash = "test_hash_123";
        bytes32 returnedHash = storageContract.getFileHash(0);
        Assert.equal(returnedHash, expectedHash, "Retrieved hash should match the expected hash");
    }

    function testDuplicateFileHash() public {
        bytes32 duplicateHash = "test_hash_123";
        bool result = storageContract.addFileHash(duplicateHash);
        Assert.isFalse(result, "Adding duplicate hash should fail");
    }

    function testAddEmptyFileHash() public {
        bytes32 emptyHash = "";
        (bool result, ) = address(storageContract).call(abi.encodePacked(storageContract.addFileHash.selector, emptyHash));
        Assert.isFalse(result, "Adding an empty hash should fail");
    }

}