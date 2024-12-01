use borsh::{BorshDeserialize, BorshSerialize};

pub(crate) const AMM_INFO_SIZE: usize = std::mem::size_of::<AmmInfoProto>();

#[derive(BorshDeserialize, BorshSerialize, ::prost::Message)]
pub struct AmmInfoProto {
    #[prost(uint64, tag = "1")]
    pub status: u64,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    #[prost(uint64, tag = "3")]
    pub order_num: u64,
    #[prost(uint64, tag = "4")]
    pub depth: u64,
    #[prost(uint64, tag = "5")]
    pub coin_decimals: u64,
    #[prost(uint64, tag = "6")]
    pub pc_decimals: u64,
    #[prost(uint64, tag = "7")]
    pub state: u64,
    #[prost(uint64, tag = "8")]
    pub reset_flag: u64,
    #[prost(uint64, tag = "9")]
    pub min_size: u64,
    #[prost(uint64, tag = "10")]
    pub vol_max_cut_ratio: u64,
    #[prost(uint64, tag = "11")]
    pub amount_wave: u64,
    #[prost(uint64, tag = "12")]
    pub coin_lot_size: u64,
    #[prost(uint64, tag = "13")]
    pub pc_lot_size: u64,
    #[prost(uint64, tag = "14")]
    pub min_price_multiplier: u64,
    #[prost(uint64, tag = "15")]
    pub max_price_multiplier: u64,
    #[prost(uint64, tag = "16")]
    pub sys_decimal_value: u64,
    #[prost(message, tag = "17")]
    pub fees: Option<FeesProto>,
    #[prost(message, tag = "18")]
    pub out_put: Option<OutPutDataProto>,
    #[prost(string, tag = "19")]
    pub token_coin: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub token_pc: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub lp_mint: ::prost::alloc::string::String,
    #[prost(string, tag = "24")]
    pub open_orders: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub market: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub serum_dex: ::prost::alloc::string::String,
    #[prost(string, tag = "27")]
    pub target_orders: ::prost::alloc::string::String,
    #[prost(string, tag = "28")]
    pub withdraw_queue: ::prost::alloc::string::String,
    #[prost(string, tag = "29")]
    pub token_temp_lp: ::prost::alloc::string::String,
    #[prost(string, tag = "30")]
    pub amm_owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "31")]
    pub lp_amount: u64,
    #[prost(uint64, tag = "32")]
    pub client_order_id: u64,
    #[prost(fixed64, repeated, tag = "33")]
    pub padding: Vec<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, ::prost::Message)]
pub struct OutPutDataProto {
    #[prost(uint64, tag = "1")]
    pub need_take_pnl_coin: u64,
    #[prost(uint64, tag = "2")]
    pub need_take_pnl_pc: u64,
    #[prost(uint64, tag = "3")]
    pub total_pnl_pc: u64,
    #[prost(uint64, tag = "4")]
    pub total_pnl_coin: u64,
    #[prost(uint64, tag = "5")]
    pub pool_open_time: u64,
    #[prost(uint64, tag = "6")]
    pub punish_pc_amount: u64,
    #[prost(uint64, tag = "7")]
    pub punish_coin_amount: u64,
    #[prost(uint64, tag = "8")]
    pub orderbook_to_init_time: u64,
    #[prost(string, tag = "9")]
    pub swap_coin_in_amount: String,
    #[prost(string, tag = "10")]
    pub swap_pc_out_amount: String,
    #[prost(uint64, tag = "11")]
    pub swap_take_pc_fee: u64,
    #[prost(string, tag = "12")]
    pub swap_pc_in_amount: String,
    #[prost(string, tag = "13")]
    pub swap_coin_out_amount: String,
    #[prost(uint64, tag = "14")]
    pub swap_take_coin_fee: u64,
}

pub(crate) const FEES_SIZE: usize = std::mem::size_of::<FeesProto>();
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, ::prost::Message)]
pub struct FeesProto {
    #[prost(uint64, tag = "1")]
    pub min_separate_numerator: u64,
    #[prost(uint64, tag = "2")]
    pub min_separate_denominator: u64,
    #[prost(uint64, tag = "3")]
    pub trade_fee_numerator: u64,
    #[prost(uint64, tag = "4")]
    pub trade_fee_denominator: u64,
    #[prost(uint64, tag = "5")]
    pub pnl_numerator: u64,
    #[prost(uint64, tag = "6")]
    pub pnl_denominator: u64,
    #[prost(uint64, tag = "7")]
    pub swap_fee_numerator: u64,
    #[prost(uint64, tag = "8")]
    pub swap_fee_denominator: u64,
}

pub(crate) const TARGET_ORDERS_SIZE: usize = std::mem::size_of::<TargetOrdersProto>();
#[derive(BorshDeserialize, BorshSerialize, ::prost::Message)]
pub struct TargetOrdersProto {
    #[prost(uint64, repeated, tag = "1")]
    pub owner: Vec<u64>,
    #[prost(message, repeated, tag = "2")]
    pub buy_orders: Vec<TargetOrderProto>,
    #[prost(fixed64, repeated, tag = "3")]
    pub padding1: Vec<u64>,
    #[prost(string, tag = "4")]
    pub target_x: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub target_y: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub plan_x_buy: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub plan_y_buy: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub plan_x_sell: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub plan_y_sell: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub placed_x: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub placed_y: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub calc_pnl_x: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub calc_pnl_y: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "14")]
    pub sell_orders: Vec<TargetOrderProto>,
    #[prost(fixed64, repeated, tag = "15")]
    pub padding2: Vec<u64>,
    #[prost(fixed64, repeated, tag = "16")]
    pub replace_buy_client_id: Vec<u64>,
    #[prost(fixed64, repeated, tag = "17")]
    pub replace_sell_client_id: Vec<u64>,
    #[prost(uint64, tag = "18")]
    pub last_order_numerator: u64,
    #[prost(uint64, tag = "19")]
    pub last_order_denominator: u64,
    #[prost(uint64, tag = "20")]
    pub plan_orders_cur: u64,
    #[prost(uint64, tag = "21")]
    pub place_orders_cur: u64,
    #[prost(uint64, tag = "22")]
    pub valid_buy_order_num: u64,
    #[prost(uint64, tag = "23")]
    pub valid_sell_order_num: u64,
    #[prost(fixed64, repeated, tag = "24")]
    pub padding3: Vec<u64>,
    #[prost(string, tag = "25")]
    pub free_slot_bits: ::prost::alloc::string::String,
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, ::prost::Message)]
pub struct TargetOrderProto {
    #[prost(uint64, tag = "1")]
    pub price: u64,
    #[prost(uint64, tag = "2")]
    pub vol: u64,
}
