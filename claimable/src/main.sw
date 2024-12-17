predicate;

use std::{
    tx::{
        tx_witness_data,
        tx_witnesses_count,
        tx_id
    },
    constants::ZERO_B256,
    b512::B512,
    ecr::ec_recover_address
};

configurable {
    CLAIMS_CONTRACT_ADDRESS: Address = Address::from(ZERO_B256),
    OWNER: Address = Address::from(ZERO_B256),
}

fn has_owner_signature() -> bool {
    if (tx_witnesses_count() < 1) {
        return false;
    }

    let current_signature = tx_witness_data::<B512>(0).unwrap();
    let current_address = ec_recover_address(current_signature, tx_id()).unwrap();

    current_address == OWNER
}

    
fn main() -> bool {
    has_owner_signature()
}
