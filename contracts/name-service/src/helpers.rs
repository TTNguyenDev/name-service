use crate::error::ContractError;
use cosmwasm_std::Coin;

pub fn assert_sent_sufficient_coin(
    sent: &[Coin],
    required: Option<Coin>
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
            if let Some(coin) =  sent_sufficient_funds.next() {
                return Ok(Some(coin));
            } else {
                return Err(ContractError::InsufficientFundsSend{})
            }
        }
    }

    Ok(None)
}
