use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryCountResponse)]
    QueryCount {}
}

#[cw_serde]
pub struct QueryCountResponse {
    pub count: u32
}

#[cw_serde]
pub enum IbcExecuteMsg {
    IncrementCount {
        tx_author: String
    },
}
