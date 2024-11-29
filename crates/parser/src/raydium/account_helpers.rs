use borsh::{BorshDeserialize, BorshSerialize};

pub(crate) const AMM_INFO_SIZE: usize = std::mem::size_of::<AmmInfo>();

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct AmmInfo {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub coin_decimals: u64,
    pub pc_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub sys_decimal_value: u64,
    pub fees: Fees,
    pub out_put: OutPutData,
    pub token_coin: solana_sdk::pubkey::Pubkey,
    pub token_pc: solana_sdk::pubkey::Pubkey,
    pub coin_mint: solana_sdk::pubkey::Pubkey,
    pub pc_mint: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub serum_dex: solana_sdk::pubkey::Pubkey,
    pub target_orders: solana_sdk::pubkey::Pubkey,
    pub withdraw_queue: solana_sdk::pubkey::Pubkey,
    pub token_temp_lp: solana_sdk::pubkey::Pubkey,
    pub amm_owner: solana_sdk::pubkey::Pubkey,
    pub lp_amount: u64,
    pub client_order_id: u64,
    pub padding: [u64; 2],
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct OutPutData {
    pub need_take_pnl_coin: u64,
    pub need_take_pnl_pc: u64,
    pub total_pnl_pc: u64,
    pub total_pnl_coin: u64,
    pub pool_open_time: u64,
    pub punish_pc_amount: u64,
    pub punish_coin_amount: u64,
    pub orderbook_to_init_time: u64,
    pub swap_coin_in_amount: u128,
    pub swap_pc_out_amount: u128,
    pub swap_take_pc_fee: u64,
    pub swap_pc_in_amount: u128,
    pub swap_coin_out_amount: u128,
    pub swap_take_coin_fee: u64,
}

pub(crate) const FEES_SIZE: usize = std::mem::size_of::<Fees>();
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Fees {
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
}

pub(crate) const TARGET_ORDERS_SIZE: usize = std::mem::size_of::<TargetOrders>();
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct TargetOrders {
    pub owner: [u64; 4],
    pub buy_orders: [TargetOrder; 50],
    pub padding1: [u64; 8],
    pub target_x: u128,
    pub target_y: u128,
    pub plan_x_buy: u128,
    pub plan_y_buy: u128,
    pub plan_x_sell: u128,
    pub plan_y_sell: u128,
    pub placed_x: u128,
    pub placed_y: u128,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    pub sell_orders: [TargetOrder; 50],
    pub padding2: [u64; 6],
    pub replace_buy_client_id: [u64; 10],
    pub replace_sell_client_id: [u64; 10],
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,
    pub plan_orders_cur: u64,
    pub place_orders_cur: u64,
    pub valid_buy_order_num: u64,
    pub valid_sell_order_num: u64,
    pub padding3: [u64; 10],
    pub free_slot_bits: u128,
}
#[derive(Debug, BorshDeserialize, BorshSerialize)]

pub struct TargetOrder {
    pub price: u64,
    pub vol: u64,
}
