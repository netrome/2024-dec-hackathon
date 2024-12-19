script;

use std::logging::log;
use std::constants::ZERO_B256;
use std::auth::msg_sender;
use claims_contract_abi::ClaimsContract;

configurable {
    CLAIMS_CONTRACT_ADDRESS: b256 = ZERO_B256,
    OWNER: Address = Address::from(ZERO_B256),
}

fn main(recipient: Address, gas: u64, coins: u64, asset_id: b256) -> u64 {
    let caller = abi(ClaimsContract, CLAIMS_CONTRACT_ADDRESS);

    caller.initiate_claim{gas, coins, asset_id}(OWNER, recipient)
}
