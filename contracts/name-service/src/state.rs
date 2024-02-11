use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
    pub cur_price: Coin,
}

pub const PURCHASE_PRICE: Item<Option<Coin>> = Item::new("purchase_price");
pub const NAME_RESOLVER: Map<&[u8], NameRecord> = Map::new("name_resolver");
