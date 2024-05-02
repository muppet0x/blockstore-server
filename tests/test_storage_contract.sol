pragma solidity >=0.4.22 <0.9.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/StorageContract.sol";

contract TestStorageContract {
    StorageContract storageContract = StorageContract(DeployedAddresses.StorageContract());

    function testAddFileHash() public {
        bytes32 testHash = "test_hash_123";
        bool result = storageContract.addFileHash(testHash);

    }

    function testRetrieveFileHash() public {
        bytes32 expectedHash = "test_hash_123";
        bytes32 returnedHash = storageContract.getFileHash(0);

    }

    function testDuplicateFileHash() public {
        bytes32 duplicateHash = "test_hash_123";
        bool result = storageContract.addFileHash(duplicateHash);

    }

    function testAddEmptyFileHash() public {
        bytes32 emptyHash = "";
        bool result = storageContract.addFileHash(emptyHash);

    }


}