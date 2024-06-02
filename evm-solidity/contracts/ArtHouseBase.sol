// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract ArtHouseBase is ReentrancyGuard {
  address payable public owner;
  uint256 public artCounter;
  uint256 public royaltyRate; 

  struct Art {
    uint256 artId;
    uint256 price;
    uint256 rfid;
  }

  mapping (uint256 => Art) private artGallery;
  mapping (uint256 => address) private artOwners;

  event CreateArt(uint256 indexed artId, uint256 indexed rfid, uint256 indexed price);
  event PurchaseArt(uint256 indexed artId, address indexed previousOwner, address indexed newOwner, uint256 price);
  event RoyaltyPaid(uint256 indexed artId, address indexed owner, uint256 royaltyPaid);

  modifier artworkExists(uint256 _artId) {
    require(_artId < artCounter, "Art does not exist!");
    _;
  }
  
  constructor(uint256 _royaltyRate) {
    owner = payable(msg.sender);
    require(_royaltyRate <= 100, "Royalty rate must be between 0 and 100");
    royaltyRate = _royaltyRate;
  }

  function createArt(uint256 _price, uint256 _rfid) public {
    require(owner == msg.sender, "Not contract owner!");
    require(_price > 0, "Price must be greater than zero");
    require(_rfid > 0, "RFID must be greater than zero");

    artGallery[artCounter] = Art(artCounter, _price, _rfid);
    artOwners[artCounter] = msg.sender;

    emit CreateArt(artCounter, _rfid, _price);
    artCounter++;
  }

  function purchaseArt(uint256 _artId) payable public artworkExists(_artId) nonReentrant {
    require (msg.value >= artGallery[_artId].price, "Insufficient balance!");

    uint256 royaltyAmount = (msg.value * royaltyRate) / 100;
    (bool royaltySent, ) = owner.call{value: royaltyAmount}("");
    require (royaltySent, "Failed to send royalties");
    emit RoyaltyPaid(_artId, owner, royaltyAmount);

    uint256 remainingAmount = msg.value - royaltyAmount;
    address previousOwner = artOwners[_artId];
    (bool paymentSent, ) = previousOwner.call{value: remainingAmount}("");
    require (paymentSent, "Failed to send payment");

    artOwners[_artId] = msg.sender;
    emit PurchaseArt(_artId, previousOwner, msg.sender, msg.value);
  }

  function getArt(uint256 _artId) public view artworkExists(_artId) returns (Art memory)  {
    require(_artId < artCounter, "Artwork does not exist");
    return artGallery[_artId];
  }

  function getArtOwner(uint256 _artId) public view artworkExists(_artId) returns (address) {
    require(_artId < artCounter, "Artwork does not exist");
    return artOwners[_artId];
  }
}