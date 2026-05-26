# JeepneyPass

Micropayment fare system for jeepneys using Stellar.

## Problem
Drivers lose money and time because commuters often lack exact fare cash.

## Solution
Passengers pay fares instantly through Stellar QR payments and Soroban fare splitting.

## Timeline
- Day 1–2: Smart contract
- Day 3–4: QR payment UI
- Day 5–6: Wallet integration
- Day 7: Testnet demo

## Stellar Features Used
- USDC transfers
- Soroban smart contracts
- Custom tokens
- Trustlines

## Vision and Purpose
Modernize public transportation payments for low-income commuters.

## Prerequisites
- Rust
- Soroban CLI v22+

## Build
```bash
soroban contract build
```