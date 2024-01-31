use std::fmt::write;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::helpers::assert_sent_sufficient_coin;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
use crate::state::{Config, NameRecord, CONFIG, NAME_RESOLVER};

const MIN_LENGTH: u64 = 3;
const MAX_LENGTH: u64 = 64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        purchase_price: msg.purchase_price,
        transfer_price: msg.transfer_price,
    };
    CONFIG.save(deps.storage, &config)?;

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
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.purchase_price)?;

    let key = name.as_bytes();
    let record = NameRecord { owner: info.sender };

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
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.transfer_price)?;

    let new_owner = deps.api.addr_validate(&to)?;
    let key = name.as_bytes();
    NAME_RESOLVER.update(deps.storage, key, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            record.owner = new_owner.clone();
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
        QueryMsg::ResolveREcord { name } => {
            let key = name.as_bytes();

            let address = match NAME_RESOLVER.may_load(deps.storage, key)? {
                Some(record) => Some(String::from(&record.owner)),
                None => None,
            };
            let resp = ResolveRecordResponse { address };
            to_binary(&resp)
        }
        QueryMsg::Config {} => to_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
    }
}

fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}

fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;
    if length < MIN_LENGTH {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_LENGTH,
        })
    } else if length > MAX_LENGTH {
        Err(ContractError::NameTooLong {
            length,
            max_length: MAX_LENGTH,
        })
    } else {
        match name.find(invalid_char) {
            None => Ok(()),
            Some(bytepos_invalid_char_start) => {
                let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}
