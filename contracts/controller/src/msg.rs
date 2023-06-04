use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {
        channel: String
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryAddressResponse)]
    QueryAddress {
        addr: String
    }
}

#[cw_serde]
pub enum IbcExecuteMsg {
    IncrementCount {
        tx_author: String
    },
}

#[cw_serde]
pub struct QueryAddressResponse {
    pub increment_requests: u64
}