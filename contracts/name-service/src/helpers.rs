use crate::error::ContractError;
use cosmwasm_std::Coin;

pub fn assert_sent_sufficient_coin(
    sent: &[Coin],
    required: Option<Coin>
) -> Result<(), ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let sent_sufficient_funds = sent.iter().any(|coin| {
                coin.denom = required_coin.denom && coin.amount.u128() > required_amount
            });
            if sent_sufficient_funds {
                return Ok(());
            } else {
                return Err(ContractError::InsufficientFundSend {})
            }
        }
    }

    Ok(())
};
