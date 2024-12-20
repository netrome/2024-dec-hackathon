use clap::Parser;

#[cfg(feature = "ssr")]
#[derive(Parser)]
pub struct Args {
    /// URL of Fuel node to connect to
    #[arg(long, env)]
    pub provider_url: String,

    /// Private key - shh, don't tell anyone!!!
    #[arg(long, env)]
    pub private_key: String,

    /// ID of claims contract. Will deploy a new one if not provided.
    #[arg(long, env)]
    pub contract_id: Option<String>,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
