use super::{EthHandler, EthTraitError};
use crate::{EvmTrait, Frame, FrameResult};
use context_interface::{result::HaltReason, ContextTrait, Journal};
use interpreter::FrameInput;
use primitives::Log;
use state::EvmState;
use std::vec::Vec;

pub struct MainnetHandler<CTX, ERROR, FRAME> {
    pub _phantom: core::marker::PhantomData<(CTX, ERROR, FRAME)>,
}

impl<EVM, ERROR, FRAME> EthHandler for MainnetHandler<EVM, ERROR, FRAME>
where
    EVM: EvmTrait<Context: ContextTrait<Journal: Journal<FinalOutput = (EvmState, Vec<Log>)>>>,
    ERROR: EthTraitError<EVM>,
    // TODO `FrameResult` should be a generic trait.
    // TODO `FrameInit` should be a generic.
    FRAME: Frame<Evm = EVM, Error = ERROR, FrameResult = FrameResult, FrameInit = FrameInput>,
{
    type Evm = EVM;
    type Error = ERROR;
    type Frame = FRAME;
    type HaltReason = HaltReason;
}

impl<CTX, ERROR, FRAME> Default for MainnetHandler<CTX, ERROR, FRAME> {
    fn default() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}
