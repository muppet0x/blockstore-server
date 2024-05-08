pragma solidity ^0.8.0;

contract StorageContract {
    event FileStored(
        address indexed owner,
        bytes32 indexed fileHash,
        string metadata,
        uint256 timestamp
    );

    event FileUpdated(
        address indexed owner,
        bytes32 indexed fileHash,
        string newMetadata,
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

    function updateFileMetadata(bytes32 fileHash, string memory newMetadata) public {
        require(files[fileHash].timestamp != 0, "File hash not stored.");
        FileDetails storage file = files[fileHash];
        require(msg.sender == file.owner, "Only the owner can update metadata.");
        file.metadata = newMetadata;
        emit FileUpdated(msg.sender, fileHash, newMetadata, block.timestamp);
    }

    function retrieveFileDetails(bytes32 fileHash) public view 
        returns (
            address, 
            uint256, 
            string memory 
        ) 
    {
        FileDetails storage details = files[fileHash];
        require(details.timestamp != 0, "File hash not found.");
        return (details.owner, details.timestamp, details.metadata);
    }

    function verifyFileHash(bytes32 fileHash) public {
        bool exists = files[fileHash].timestamp != 0;
        emit FileVerified(msg.sender, fileHash, exists);
    }
}