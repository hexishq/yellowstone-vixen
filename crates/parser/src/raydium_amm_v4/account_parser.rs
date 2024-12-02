use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use yellowstone_vixen_core::{AccountUpdate, ParseResult, Parser, Prefilter, ProgramParser};

use super::{
    account_helpers::{AmmInfo, Fees, TargetOrders, AMM_INFO_SIZE, FEES_SIZE, TARGET_ORDERS_SIZE},
    instruction_parser::RAYDIUM_AMM_V4_POOL,
};
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
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
        "yellowstone_vixen_parser::raydium::AccountParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        Prefilter::builder()
            .account_owners([RAYDIUM_AMM_V4_POOL])
            .build()
            .unwrap()
    }

    async fn parse(&self, acc_update: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acc_update
            .account
            .as_ref()
            .ok_or(ProgramError::InvalidArgument)?;

        self.unpack(&mut inner.data.as_slice())
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        RAYDIUM_AMM_V4_POOL.to_bytes().into()
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
