use yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction;
use yellowstone_vixen_core::Parser;

use super::instruction_helpers::{RaydiumInstruction, RAYDIUM_AMM_V4_POOL};

pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = SubscribeUpdateTransaction;

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

    fn parse(
        &self,
        value: &Self::Input,
    ) -> impl std::future::Future<Output = yellowstone_vixen_core::ParseResult<Self::Output>> + Send
    {
        todo!()
    }
}
