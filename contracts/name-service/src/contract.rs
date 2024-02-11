use std::fmt::write;

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::helpers::*;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
use crate::state::{NameRecord, NAME_RESOLVER, PURCHASE_PRICE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PURCHASE_PRICE.save(deps.storage, &msg.purchase_price)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => execute_register(deps, info, name),
        ExecuteMsg::Transfer { name, to } => execute_transfer(deps, info, name, to),
    }
}

pub fn execute_register(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    validate_name(&name)?;
    let purchase_price = PURCHASE_PRICE.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, purchase_price.clone())?;

    let key = name.as_bytes();
    let record = NameRecord {
        owner: info.sender,
        cur_price: purchase_price.unwrap(),
    };

    if (NAME_RESOLVER.may_load(deps.storage, key)?).is_some() {
        return Err(ContractError::NameTaken { name });
    }

    NAME_RESOLVER.save(deps.storage, key, &record)?;
    Ok(Response::default())
}

pub fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    to: String,
) -> Result<Response, ContractError> {
    let key = name.as_bytes();

    let record = if let Some(data) = NAME_RESOLVER.may_load(deps.storage, key)? {
        data
    } else {
        return Err(ContractError::NameNotExists { name: name.clone() });
    };

    let coin = assert_sent_sufficient_coin(&info.funds, Some(record.cur_price))?;
    let new_owner = deps.api.addr_validate(&to)?;
    NAME_RESOLVER.update(deps.storage, key, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            record.owner = new_owner.clone();
            record.cur_price = coin.unwrap().clone();
            Ok(record)
        } else {
            Err(ContractError::NameNotExists { name: name.clone() })
        }
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { name } => {
            let key = name.as_bytes();

            let address = match NAME_RESOLVER.may_load(deps.storage, key)? {
                Some(record) => Some(String::from(&record.owner)),
                None => None,
            };
            let resp = ResolveRecordResponse { address };
            to_json_binary(&resp)
        }
    }
}
