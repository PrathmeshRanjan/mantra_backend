# Stockholm CosmWasm Smart Contracts

** Frontend repo ** - https://github.com/PrathmeshRanjan/mantra-frontend

This document provides an overview of four CosmWasm smart contracts designed for a Real-World Asset (RWA) platform. The contracts facilitate various functionalities such as trading NFTs representing RWAs, swapping gold tokens with OM tokens, managing a liquidity pool, and staking RWAs for rewards.

## 1. RWA NFT Trading Contract

### Functions:
- **Mint NFTs**: Represent RWAs as NFTs on the blockchain.
- **List NFTs for Sale**: Allow NFT owners to list their NFTs for sale, specifying a price.
- **Buy NFTs**: Enable users to purchase listed NFTs, transferring ownership and handling payment.

### Operation:
The contract uses the CW721 base for NFT functionality, extending it with sale listing and buying features. Ownership verification is performed before listing, ensuring that only the NFT owner can initiate a sale. The purchase function transfers funds from the buyer to the seller and updates the NFT ownership.

### Production Readiness:
- Implement robust error handling and input validation to ensure security and data integrity.
- Integrate with an oracle or a trusted external system for verifying RWAs tied to NFTs.
- Add event logging for critical actions to facilitate monitoring and auditing.

## 2. Gold-OM Token Swap Contract

### Functions:
- **Set Exchange Rate**: Allow the admin to set the exchange rate between gold tokens and OM tokens.
- **Swap Tokens**: Users can swap their gold tokens for OM tokens based on the current exchange rate.

### Operation:
The contract maintains an exchange rate state variable that the admin can update. When users send gold tokens to the contract, it calculates the equivalent OM tokens using the exchange rate and transfers the OM tokens to the user's account.

### Production Readiness:
- Implement rate-limiting and slippage control to protect against market manipulation and flash crashes.
- Introduce a mechanism for dynamic exchange rate adjustment based on market conditions or through a decentralized oracle.
- Ensure compliance with legal and regulatory requirements for token swaps and financial transactions.

## 3. Liquidity Pool Contract

### Functions:
- **Deposit Tokens**: Users can deposit OM tokens or RWAs (represented as tokens) into the liquidity pool.
- **Withdraw Assets**: Allow users to withdraw their deposited assets from the pool.

### Operation:
The contract tracks the total pool balances for OM tokens and RWAs. Users can add to the pool by depositing assets, which updates the pool's balance. Withdrawals are processed by deducting from the pool balance and transferring assets back to the user.

### Production Readiness:
- Implement liquidity provider (LP) tokens to represent ownership in the pool, enabling fair distribution of fees and rewards.
- Add functionality for yield farming or earning interest on deposited assets to incentivize liquidity provision.
- Incorporate security measures like timelocks and multi-sig requirements for critical administrative functions.

## 4. RWA Staking Contract

### Functions:
- **Stake NFTs**: Users can stake their NFTs representing RWAs to earn rewards.
- **Unstake NFTs**: Allow users to unstake their NFTs and stop earning rewards.
- **Claim Rewards**: Users can claim their accrued rewards in OM tokens.

### Operation:
The contract allows users to stake NFTs by locking them into the contract. A staking duration is tracked for each NFT, which determines the reward amount based on a predefined rate. Users can unstake their NFTs and claim their rewards, which are paid out in OM tokens.

### Production Readiness:
- Introduce a mechanism for adjusting staking rewards based on total staked assets and pool performance.
- Add safeguards against re-entrancy attacks and ensure contract interactions are secure.
- Consider integrating with insurance or collateralization services to protect staked assets and guarantee reward payouts.
