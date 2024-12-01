use borsh::BorshDeserialize;
use solana_program::{pubkey, pubkey::Pubkey};
use yellowstone_vixen_core::{instruction::InstructionUpdate, ParseResult, Parser};

pub const RAYDIUM_AMM_V4_POOL: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

use crate::helpers::IX_DISCRIMINATOR_SIZE;

use super::instruction_helpers::*;

#[derive(Clone, Copy, Debug)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = AmmInstruction;

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
    ) -> Result<AmmInstruction, yellowstone_vixen_core::ParseError> {
        let ix_discriminator: [u8; 8] = ix.data[0..IX_DISCRIMINATOR_SIZE].try_into()?;
        let mut ix_data = &ix.data[IX_DISCRIMINATOR_SIZE..];

        match ix_discriminator {
            SWAP_BASE_IN_IX_DISC => {
                let swap_base_in: SwapBaseIn = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(AmmInstruction::SwapBaseIn(SwapBaseIn {
                    amount_in: swap_base_in.amount_in,
                    minimum_amount_out: swap_base_in.minimum_amount_out,
                }))
            },
            SWAP_BASE_OUT_IX_DISC => {
                let swap_base_out: SwapBaseOut =
                    BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(AmmInstruction::SwapBaseOut(SwapBaseOut {
                    max_amount_in: swap_base_out.max_amount_in,
                    amount_out: swap_base_out.amount_out,
                }))
            },
            WITHDRAW_IX_DISC => {
                let withdraw: Withdraw = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(AmmInstruction::Withdraw(Withdraw {
                    amount: withdraw.amount,
                    min_coin_amount: withdraw.min_coin_amount,
                    min_pc_amount: withdraw.min_pc_amount,
                }))
            },
            DEPOSIT_IX_DISC => {
                let deposit: Deposit = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(AmmInstruction::Deposit(Deposit {
                    max_coin_amount: deposit.max_coin_amount,
                    max_pc_amount: deposit.max_pc_amount,
                    base_side: deposit.base_side,
                    other_amount_min: deposit.other_amount_min,
                }))
            },
            INITIALIZE2_IX_DISC => {
                let initialize2: Initialize2 = BorshDeserialize::deserialize(&mut ix_data).unwrap();
                Ok(AmmInstruction::Initialize2(Initialize2 {
                    nonce: initialize2.nonce,
                    open_time: initialize2.open_time,
                    init_pc_amount: initialize2.init_pc_amount,
                    init_coin_amount: initialize2.init_coin_amount,
                }))
            },
            _ => Err(yellowstone_vixen_core::ParseError::Other(
                "Unknown instruction".into(),
            )),
        }
    }
}
