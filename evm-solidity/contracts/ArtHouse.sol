// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract ArtHouse is Ownable, ReentrancyGuard {
    uint256 public nextArtId;
    uint256 public nextCollectionId;

    struct Art {
        uint256 id;
        uint256 price;
        string rfidId;
        string name;
        bool forSale;
        address owner;
    }

    struct Collection {
        uint256 id;
        string name;
        uint256[] artIds;
    }

    mapping(uint256 => Art) public arts;
    mapping(uint256 => Collection) public collections;
    mapping(address => uint256[]) public ownerToArtIds;

    event ArtCreated(uint256 artId, uint256 price, string rfidId, string name);
    event CollectionCreated(uint256 collectionId, string name);
    event ArtAddedToCollection(uint256 artId, uint256 collectionId);
    event ArtForSale(uint256 artId, uint256 price);
    event ArtPurchased(uint256 artId, address buyer);
    event OwnershipTransferred(address previousOwner, address newOwner, uint256 artId);

    modifier onlyArtOwner(uint256 _artId) {
        require(arts[_artId].owner == msg.sender, "Not the owner");
        _;
    }

    modifier artExists(uint256 _artId) {
        require(_artId < nextArtId, "Art does not exist");
        _;
    }

    modifier collectionExists(uint256 _collectionId) {
        require(_collectionId < nextCollectionId, "Collection does not exist");
        _;
    }

    constructor() Ownable(msg.sender) {
    }

    function createArt(uint256 _price, string memory _rfidId, string memory _name) public {
        require(_price > 0, "Price must be greater than zero");
        require(bytes(_rfidId).length > 0, "RFID ID is required");
        require(bytes(_name).length > 0, "Name is required");

        uint256 artId = nextArtId++;
        arts[artId] = Art(artId, _price, _rfidId, _name, false, msg.sender);
        ownerToArtIds[msg.sender].push(artId);
        emit ArtCreated(artId, _price, _rfidId, _name);
    }

    function createCollection(string memory _name) public onlyOwner {
        require(bytes(_name).length > 0, "Name is required");

        uint256 collectionId = nextCollectionId++;
        collections[collectionId] = Collection(collectionId, _name, new uint256[](0));
        emit CollectionCreated(collectionId, _name);
    }

    function addArtToCollection(uint256 _artId, uint256 _collectionId) 
        public 
        onlyArtOwner(_artId) 
        artExists(_artId) 
        collectionExists(_collectionId) 
    {
        collections[_collectionId].artIds.push(_artId);
        emit ArtAddedToCollection(_artId, _collectionId);
    }

    function sellArt(uint256 _artId, uint256 _price) 
        public 
        onlyArtOwner(_artId) 
        artExists(_artId) 
    {
        require(_price > 0, "Price must be greater than zero");
        
        arts[_artId].forSale = true;
        arts[_artId].price = _price;
        emit ArtForSale(_artId, _price);
    }

    function purchaseArt(uint256 _artId) 
        public 
        payable 
        nonReentrant 
        artExists(_artId) 
    {
        require(arts[_artId].forSale, "Art not for sale");
        require(msg.value >= arts[_artId].price, "Insufficient payment");

        address previousOwner = arts[_artId].owner;
        arts[_artId].owner = msg.sender;
        arts[_artId].forSale = false;

        (bool sent, ) = payable(previousOwner).call{value: msg.value}("");
        require(sent, "Failed to send Ether");

        _removeArtFromOwner(previousOwner, _artId);
        ownerToArtIds[msg.sender].push(_artId);

        emit ArtPurchased(_artId, msg.sender);
        emit OwnershipTransferred(previousOwner, msg.sender, _artId);
    }

    function withdrawFunds() public onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "No funds to withdraw");

        (bool sent, ) = msg.sender.call{value: balance}("");
        require(sent, "Failed to send Ether");
    }

    function _removeArtFromOwner(address _owner, uint256 _artId) internal {
        uint256[] storage artIds = ownerToArtIds[_owner];
        for (uint256 i = 0; i < artIds.length; i++) {
            if (artIds[i] == _artId) {
                artIds[i] = artIds[artIds.length - 1];
                artIds.pop();
                break;
            }
        }
    }
}
