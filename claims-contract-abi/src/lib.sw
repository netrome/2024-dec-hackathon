library;

// anything `pub` here will be exported as a part of this library's API

pub struct Claim {
    pub id: u64,
    pub owner: Address,
    pub recipient: Address,
    pub asset: AssetId,
    pub amount: u64,
    pub block_height: u32,
}

abi ClaimsContract {
    #[storage(read, write), payable]
    fn initiate_claim(owner: Address, recipient: Address) -> u64;

    #[storage(read, write)]
    fn disprove(claim_id: u64);

    #[storage(read, write)]
    fn fulfill(claim_id: u64);

    // 🤔 Do we really need this method on-chain?
    #[storage(read)]
    fn get_claims(addr: Address) -> Vec<Claim>;
}
