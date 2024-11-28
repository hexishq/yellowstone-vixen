use solana_program::{pubkey, pubkey::Pubkey};

pub const RAYDIUM_AMM_V4_POOL: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

#[derive(Debug, Clone, Copy)]
pub enum RaydiumInstruction {
    SwapBaseIn(SwapBaseIn),
    SwapBaseOut(SwapBaseOut),
}

#[derive(Debug, Clone, Copy)]
pub struct SwapBaseIn {}

#[derive(Debug, Clone, Copy)]
pub struct SwapBaseOut {}
