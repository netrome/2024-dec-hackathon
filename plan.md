# Project: Quarantine enabled wallet

## Functionality
* View balances
* Spend coins
* Claim funds
* Challenge claims

## Components

* **Wallet app**: Web app that displays balances, makes transfers, manages claims etc.
* **Claims contract**: Contains claimed funds.
* **Claimable predicate**: Special predicate with two spend paths:
  * Private key spend. Can be used in any transaction.
  * Claim spend. Has to put the money in the claims contract plus an additional claim fee.

## Implementation plan

Milestone 1: Claims contract + tests.
Milestone 2: Claimable predicate + tests (integration with contract).
Milestone 3: Wallet app.
