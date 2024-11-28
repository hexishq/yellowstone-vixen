use borsh::BorshDeserialize;
use solana_program::{pubkey, pubkey::Pubkey};
use yellowstone_vixen_core::{instruction::InstructionUpdate, ParseResult, Parser};

pub const RAYDIUM_AMM_V4_POOL: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

use crate::helpers::IX_DISCRIMINATOR_SIZE;

use super::instruction_helpers::*;

pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = RaydiumInstruction;

    fn id(&self) -> std::borrow::Cow<str> {
        "yellowstone_vixen_parser::raydium_amm_v4::InstructionParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .transaction_accounts([RAYDIUM_AMM_V4_POOL])
            .build()
            .unwrap()
    }

    async fn parse(&self, value: &InstructionUpdate) -> ParseResult<Self::Output> {
        if value.program.equals_ref(RAYDIUM_AMM_V4_POOL) {
            InstructionParser::parse_impl(value).map_err(|e| e.into())
        } else {
            Err(yellowstone_vixen_core::ParseError::Filtered)
        }
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &InstructionUpdate,
    ) -> Result<RaydiumInstruction, yellowstone_vixen_core::ParseError> {
        let ix_discriminator: [u8; 8] = ix.data[0..IX_DISCRIMINATOR_SIZE].try_into()?;
        let mut ix_data = &ix.data[IX_DISCRIMINATOR_SIZE..];

        match ix_discriminator {
            SWAP_BASE_IN_IX_DISC => {
                let swap_base_in: SwapBaseIn = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(RaydiumInstruction::SwapBaseIn(swap_base_in))
            },
            SWAP_BASE_OUT_IX_DISC => {
                let swap_base_out: SwapBaseOut =
                    BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(RaydiumInstruction::SwapBaseOut(swap_base_out))
            },
            WITHDRAW_IX_DISC => {
                let withdraw: Withdraw = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(RaydiumInstruction::Withdraw(withdraw))
            },
            DEPOSIT_IX_DISC => {
                let deposit: Deposit = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(RaydiumInstruction::Deposit(deposit))
            },
            INITIALIZE2_IX_DISC => {
                let initialize2: Initialize2 = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(RaydiumInstruction::Initialize2(initialize2))
            },
            _ => Err(yellowstone_vixen_core::ParseError::Other(
                "Unknown instruction".into(),
            )),
        }
    }
}
