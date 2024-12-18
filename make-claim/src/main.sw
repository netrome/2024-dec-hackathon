script;

use std::logging::log;
use claims_contract::ClaimsContract;

configurable {
    SECRET_NUMBER: u64 = 0
}

fn main() -> u64 {
    log(SECRET_NUMBER);
    return SECRET_NUMBER;
}
