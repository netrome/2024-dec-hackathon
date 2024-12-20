#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct KpopInfo {
    pub base_address: String,
    pub claimable_address: String,
    pub contract_id: String,
    pub provider_url: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Claim {
    pub claim_id: u64,
    pub owner: String,
    pub recipient: String,
    pub asset_id: String,
    pub amount: u64,
    pub block_height: u32,
}
