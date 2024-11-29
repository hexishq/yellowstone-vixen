use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser};

use super::account_helpers::{
    AmmInfo, Fees, TargetOrders, AMM_INFO_SIZE, FEES_SIZE, TARGET_ORDERS_SIZE,
};

pub enum RaydiumAccountState {
    AmmInfo(AmmInfo),
    Fees(Fees),
    TargetOrder(TargetOrders),
}

#[derive(Debug, Clone, Copy)]
pub struct AccountParser {}

impl Parser for AccountParser {
    type Input = AccountUpdate;

    type Output = RaydiumAccountState;

    fn id(&self) -> std::borrow::Cow<str> {
        todo!()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        todo!()
    }

    async fn parse(&self, acc_update: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acc_update
            .account
            .as_ref()
            .ok_or(ProgramError::InvalidArgument)?;

        self.unpack(&mut inner.data.as_slice())
    }
}

impl AccountParser {
    pub(crate) fn unpack(&self, acc_data: &mut &[u8]) -> ParseResult<RaydiumAccountState> {
        match acc_data.len() {
            AMM_INFO_SIZE => {
                let amm_info = BorshDeserialize::deserialize(acc_data)
                    .map_err(|_| ProgramError::InvalidArgument)?;
                Ok(RaydiumAccountState::AmmInfo(amm_info))
            },
            FEES_SIZE => {
                let fees = BorshDeserialize::deserialize(acc_data)
                    .map_err(|_| ProgramError::InvalidArgument)?;
                Ok(RaydiumAccountState::Fees(fees))
            },
            TARGET_ORDERS_SIZE => {
                let target_orders = BorshDeserialize::deserialize(acc_data)
                    .map_err(|_| ProgramError::InvalidArgument)?;
                Ok(RaydiumAccountState::TargetOrder(target_orders))
            },
            _ => Err(yellowstone_vixen_core::ParseError::Other(
                "Invalid account data size".into(),
            )),
        }
    }
}
