use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub purchase_price: Option<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
    Transfer { name: String, to: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ResolveRecordResponse)]
    ResolveRecord { name: String },
}

#[cw_serde]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}
