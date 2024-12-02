use borsh::BorshDeserialize;

pub const DISCRIMINATOR_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub enum AmmInstruction {
    SwapBaseIn(SwapBaseIn),
    SwapBaseOut(SwapBaseOut),
    Withdraw(Withdraw),
    Deposit(Deposit),
    Initialize2(Initialize2),
}

pub const SWAP_BASE_IN_IX_DISC: [u8; 8] = [9, 0, 0, 0, 0, 0, 0, 0];
#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct SwapBaseIn {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

pub const SWAP_BASE_OUT_IX_DISC: [u8; 8] = [11, 0, 0, 0, 0, 0, 0, 0];
#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct SwapBaseOut {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

pub const WITHDRAW_IX_DISC: [u8; 8] = [4, 0, 0, 0, 0, 0, 0, 0];
#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct Withdraw {
    pub amount: u64,
    pub min_coin_amount: Option<u64>,
    pub min_pc_amount: Option<u64>,
}

pub const DEPOSIT_IX_DISC: [u8; 8] = [3, 0, 0, 0, 0, 0, 0, 0];

#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct Deposit {
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
    pub other_amount_min: Option<u64>,
}

pub const INITIALIZE2_IX_DISC: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
#[derive(Debug, Clone, Copy, BorshDeserialize)]
pub struct Initialize2 {
    pub nonce: u8,
    pub open_time: u64,
    pub init_pc_amount: u64,
    pub init_coin_amount: u64,
}
