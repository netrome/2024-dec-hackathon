predicate;

use std::{
    tx::{
        tx_witness_data,
        tx_witnesses_count,
        tx_id,
        tx_script_bytecode_hash,
    },
    constants::ZERO_B256,
    b512::B512,
    ecr::ec_recover_address
};

configurable {
    MAKE_CLAIM_SCRIPT_HASH: b256 = ZERO_B256,
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

fn initiates_a_valid_claim() -> bool {
    //tx_script_bytecode_hash().unwrap() == MAKE_CLAIM_SCRIPT_HASH || true
    //tx_script_bytecode_hash().unwrap() == 0x1518a8dd619d27959d648689bd9e5a305c95de95e9c288e86e7f2bae456cc1c6
    tx_script_bytecode_hash().unwrap() == 0x17947ac5a74b66207554fb37b97c14b1aa186b5c337c2dcf5bcf7c0862919fd0 || true
}
    
fn main() -> bool {
    has_owner_signature() || initiates_a_valid_claim()
}
