use crate::error::ContractError;
use cosmwasm_std::Coin;

const MIN_LENGTH: u64 = 3;
const MAX_LENGTH: u64 = 64;

pub fn assert_sent_sufficient_coin(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<Option<&Coin>, ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let mut sent_sufficient_funds = sent.iter().filter_map(|coin| {
                if coin.denom == required_coin.denom && coin.amount.u128() >= required_amount {
                    Some(coin)
                } else {
                    None
                }
            });
            if let Some(coin) = sent_sufficient_funds.next() {
                return Ok(Some(coin));
            } else {
                return Err(ContractError::InsufficientFundsSend {});
            }
        }
    }

    Ok(None)
}

pub fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}

pub fn validate_name(name: &str) -> Result<(), ContractError> {
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
