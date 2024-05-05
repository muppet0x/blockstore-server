pragma solidity ^0.8.0;

contract StorageContract {
    event FileStored(
        address indexed owner,
        bytes32 indexed fileHash,
        string metadata,
        uint256 timestamp
    );

    event FileVerified(
        address indexed verifier,
        bytes32 indexed fileHash,
        bool exists
    );

    struct FileDetails {
        address owner;
        uint256 timestamp;
        string metadata;
    }

    mapping(bytes32 => FileDetails) private files;

    function storeFileHash(bytes32 fileHash, string memory metadata) public {
        require(files[fileHash].timestamp == 0, "File hash already stored.");
        files[fileHash] = FileDetails(msg.sender, block.timestamp, metadata);
        emit FileStored(msg.sender, fileHash, metadata, block.timestamp);
    }

    function retrieveFileDetails(bytes32 fileHash) public view 
        returns (
            address owner,
            uint256 timestamp,
            string memory metadata
        ) 
    {
        require(files[fileHash].timestamp != 0, "File hash not found.");
        FileDetails memory details = files[fileHash];
        return (details.owner, details.timestamp, details.metadata);
    }

    function verifyFileHash(bytes32 fileHash) public {
        bool exists = files[fileHash].timestamp != 0;
        emit FileVerified(msg.sender, fileHash, exists);
    }
}