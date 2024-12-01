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

#[cfg(feature = "proto")]
mod proto_parser {
    use super::*;
    use crate::{
        helpers::IntoProto,
        raydium::{
            account_helpers::{OutPutData, TargetOrder},
            account_proto_helpers::{
                AmmInfoProto, FeesProto, OutPutDataProto, TargetOrderProto, TargetOrdersProto,
            },
        },
    };

    impl IntoProto<AmmInfoProto> for AmmInfo {
        fn into_proto(self) -> AmmInfoProto {
            AmmInfoProto {
                status: self.status.into(),
                nonce: self.nonce.into(),
                order_num: self.order_num.into(),
                depth: self.depth.into(),
                coin_decimals: self.coin_decimals.into(),
                pc_decimals: self.pc_decimals.into(),
                state: self.state.into(),
                reset_flag: self.reset_flag.into(),
                min_size: self.min_size.into(),
                vol_max_cut_ratio: self.vol_max_cut_ratio.into(),
                amount_wave: self.amount_wave.into(),
                coin_lot_size: self.coin_lot_size.into(),
                pc_lot_size: self.pc_lot_size.into(),
                min_price_multiplier: self.min_price_multiplier.into(),
                max_price_multiplier: self.max_price_multiplier.into(),
                sys_decimal_value: self.sys_decimal_value.into(),
                fees: Some(self.fees.into_proto()),
                out_put: Some(self.out_put.into_proto()),
                token_coin: self.token_coin.to_string(),
                token_pc: self.token_pc.to_string(),
                coin_mint: self.coin_lot_size.to_string(),
                pc_mint: self.pc_mint.to_string(),
                lp_mint: self.lp_mint.to_string(),
                open_orders: self.open_orders.to_string(),
                market: self.market.to_string(),
                serum_dex: self.serum_dex.to_string(),
                target_orders: self.target_orders.to_string(),
                withdraw_queue: self.withdraw_queue.to_string(),
                token_temp_lp: self.token_temp_lp.to_string(),
                amm_owner: self.amm_owner.to_string(),
                lp_amount: self.lp_amount.into(),
                client_order_id: self.client_order_id.into(),
                padding: self.padding.into(),
            }
        }
    }

    impl IntoProto<FeesProto> for Fees {
        fn into_proto(self) -> FeesProto {
            FeesProto {
                min_separate_numerator: self.min_separate_numerator.into(),
                min_separate_denominator: self.min_separate_denominator.into(),
                trade_fee_numerator: self.trade_fee_numerator.into(),
                trade_fee_denominator: self.trade_fee_denominator.into(),
                pnl_numerator: self.pnl_numerator.into(),
                pnl_denominator: self.pnl_denominator.into(),
                swap_fee_numerator: self.swap_fee_numerator.into(),
                swap_fee_denominator: self.swap_fee_denominator.into(),
            }
        }
    }

    impl IntoProto<OutPutDataProto> for OutPutData {
        fn into_proto(self) -> OutPutDataProto {
            OutPutDataProto {
                need_take_pnl_coin: self.need_take_pnl_coin.into(),
                need_take_pnl_pc: self.need_take_pnl_pc.into(),
                total_pnl_pc: self.total_pnl_pc.into(),
                total_pnl_coin: self.total_pnl_coin.into(),
                pool_open_time: self.pool_open_time.into(),
                punish_pc_amount: self.punish_pc_amount.into(),
                punish_coin_amount: self.punish_coin_amount.into(),
                orderbook_to_init_time: self.orderbook_to_init_time.into(),
                swap_coin_in_amount: self.swap_coin_in_amount.to_string(),
                swap_pc_out_amount: self.swap_pc_out_amount.to_string(),
                swap_take_pc_fee: self.swap_take_pc_fee.into(),
                swap_pc_in_amount: self.swap_pc_in_amount.to_string(),
                swap_coin_out_amount: self.swap_coin_out_amount.to_string(),
                swap_take_coin_fee: self.swap_take_coin_fee.into(),
            }
        }
    }

    impl IntoProto<TargetOrdersProto> for TargetOrders {
        fn into_proto(self) -> TargetOrdersProto {
            TargetOrdersProto {
                owner: self.owner.into(),
                buy_orders: self
                    .buy_orders
                    .into_iter()
                    .map(IntoProto::into_proto)
                    .collect(),
                padding1: self.padding1.into(),
                target_x: self.target_x.to_string(),
                target_y: self.target_y.to_string(),
                plan_x_buy: self.plan_x_buy.to_string(),
                plan_y_buy: self.plan_y_buy.to_string(),
                plan_x_sell: self.plan_x_sell.to_string(),
                plan_y_sell: self.plan_y_sell.to_string(),
                placed_x: self.placed_x.to_string(),
                placed_y: self.placed_y.to_string(),
                calc_pnl_x: self.calc_pnl_x.to_string(),
                calc_pnl_y: self.calc_pnl_y.to_string(),
                sell_orders: self
                    .sell_orders
                    .into_iter()
                    .map(IntoProto::into_proto)
                    .collect(),
                padding2: self.padding2.into(),
                replace_buy_client_id: self.replace_buy_client_id.into(),
                replace_sell_client_id: self.replace_sell_client_id.into(),
                last_order_numerator: self.last_order_numerator.into(),
                last_order_denominator: self.last_order_denominator.into(),
                plan_orders_cur: self.plan_orders_cur.into(),
                place_orders_cur: self.place_orders_cur.into(),
                valid_buy_order_num: self.valid_buy_order_num.into(),
                valid_sell_order_num: self.valid_sell_order_num.into(),
                padding3: self.padding3.into(),
                free_slot_bits: self.free_slot_bits.to_string(),
            }
        }
    }

    impl IntoProto<TargetOrderProto> for TargetOrder {
        fn into_proto(self) -> TargetOrderProto {
            TargetOrderProto {
                price: self.price.into(),
                vol: self.vol.into(),
            }
        }
    }
}
