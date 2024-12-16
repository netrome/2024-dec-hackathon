contract;

use std::call_frames::msg_asset_id;
use std::context::msg_amount;
use std::context::this_balance;
use std::auth::msg_sender;
use std::asset::transfer;
use std::block::height;
use std::storage::storage_vec::*;

use std::hash::Hash;

struct Claim {
    id: u64,
    owner: Address,
    recipient: Address,
    asset: AssetId,
    amount: u64,
    block_height: u32,
}

storage {
    claim_counter: u64 = 0,

    claims: StorageMap<u64, Claim> = StorageMap{},
    claims_by_address: StorageMap<Address, StorageVec<Claim>> = StorageMap{},
}

enum InvalidError {
    OnlyOwner: Address,
    OnlyRecipient: Address,
    NotEnoughTokens: u64,
    TooSoon: u32,
}

abi ClaimsContract {
    #[storage(read, write)]
    fn initiate_claim(owner: Address, recipient: Address);

    #[storage(read, write)]
    fn disprove(claim_id: u64);

    #[storage(read, write)]
    fn fulfill(claim_id: u64);

    // 🤔 Do we really need this method on-chain?
    #[storage(read)]
    fn get_claims(addr: Address) -> Vec<Claim>;
}

impl ClaimsContract for Contract {
    #[storage(read, write)]
    fn initiate_claim(owner: Address, recipient: Address) {
        let claim_id = storage.claim_counter.read();
        let block_height = 0;

        let amount = msg_amount();
        let asset = msg_asset_id();

        let claim = Claim {
            id: claim_id,
            owner,
            recipient,
            asset,
            amount,
            block_height,
        };

        storage.claims.insert(claim_id, claim);

        match storage.claims_by_address.get(owner).try_read() {
            Some(_) => (),
            None => storage.claims_by_address.insert(owner, StorageVec {}),
        };
        storage.claims_by_address.get(owner).push(claim);
        
        storage.claim_counter.write(claim_id + 1);
    }

    #[storage(read, write)]
    fn disprove(claim_id: u64) {
        let sender = msg_sender().unwrap().as_address().unwrap();
        let claim = storage.claims.get(claim_id).try_read().unwrap();

        require(sender == claim.owner, InvalidError::OnlyOwner(sender));
        
        let contract_balance = this_balance(claim.asset);
        require(contract_balance >= claim.amount, InvalidError::NotEnoughTokens(contract_balance));

        let mut idx = 0;
        let len = storage.claims_by_address.get(claim.owner).len();

        // Ugh...
        while idx < len {
            if storage.claims_by_address.get(claim.owner).get(idx).unwrap().try_read().unwrap().id == claim.id {
                let _ = storage.claims_by_address.get(claim.owner).remove(idx);
            } else {
                idx += 1;
            }
        }

        let _ = storage.claims.remove(claim_id);
        transfer(Identity::Address(claim.owner), claim.asset, claim.amount);
    }

    #[storage(read, write)]
    fn fulfill(claim_id: u64) {
        let sender = msg_sender().unwrap().as_address().unwrap();
        let claim = storage.claims.get(claim_id).try_read().unwrap();
        let min_height = claim.block_height + 3600; // TODO: Add parameter

        require(min_height <= height(), InvalidError::TooSoon(min_height));

        // 🤔 Maybe we don't need this
        require(sender == claim.recipient, InvalidError::OnlyRecipient(sender));

        let contract_balance = this_balance(claim.asset);
        require(contract_balance >= claim.amount, InvalidError::NotEnoughTokens(contract_balance));

        // Ugh...
        while idx < len {
            if storage.claims_by_address.get(claim.owner).get(idx).unwrap().try_read().unwrap().id == claim.id {
                let _ = storage.claims_by_address.get(claim.owner).remove(idx);
            } else {
                idx += 1;
            }
        }

        let _ = storage.claims.remove(claim_id);
        transfer(Identity::Address(claim.recipient), claim.asset, claim.amount);
    }

    #[storage(read)]
    fn get_claims(addr: Address) -> Vec<Claim> {
        let len = storage.claims_by_address.get(addr).len();
        let mut idx = 0;

        let mut output = Vec::new();

        while idx < len {
            let claim = storage.claims_by_address.get(addr).get(idx).unwrap().try_read().unwrap();

            output.push(claim);

            idx += 1;
        }

        output
    }
}
