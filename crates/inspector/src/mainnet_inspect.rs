use context::{ContextSetters, ContextTr, Evm, JournalOutput, JournalTr};
use database_interface::DatabaseCommit;
use handler::{
    instructions::InstructionProvider, EthFrame, EvmTr, EvmTrError, Frame, FrameResult, Handler,
    MainnetHandler, PrecompileProvider,
};
use interpreter::{interpreter::EthInterpreter, FrameInput, InterpreterResult};

use crate::{
    inspect::{InspectCommitEvm, InspectEvm},
    Inspector, InspectorEvmTr, InspectorFrame, InspectorHandler, JournalExt,
};

impl<EVM, ERROR, FRAME> InspectorHandler for MainnetHandler<EVM, ERROR, FRAME>
where
    EVM: InspectorEvmTr<
        Context: ContextTr<Journal: JournalTr<FinalOutput = JournalOutput>>,
        Inspector: Inspector<<<Self as Handler>::Evm as EvmTr>::Context, EthInterpreter>,
    >,
    ERROR: EvmTrError<EVM>,
    FRAME: Frame<Evm = EVM, Error = ERROR, FrameResult = FrameResult, FrameInit = FrameInput>
        + InspectorFrame<IT = EthInterpreter>,
{
    type IT = EthInterpreter;
}

impl<CTX, INSP, INST, PRECOMPILES> InspectEvm for Evm<CTX, INSP, INST, PRECOMPILES>
where
    CTX: ContextSetters + ContextTr<Journal: JournalTr<FinalOutput = JournalOutput> + JournalExt>,
    INSP: Inspector<CTX, EthInterpreter>,
    INST: InstructionProvider<Context = CTX, InterpreterTypes = EthInterpreter>,
    PRECOMPILES: PrecompileProvider<CTX, Output = InterpreterResult>,
{
    type Inspector = INSP;

    fn set_inspector(&mut self, inspector: Self::Inspector) {
        self.data.inspector = inspector;
    }

    fn inspect_replay(&mut self) -> Self::Output {
        let mut t = MainnetHandler::<_, _, EthFrame<_, _, _>> {
            _phantom: core::marker::PhantomData,
        };

        t.inspect_run(self)
    }
}

impl<CTX, INSP, INST, PRECOMPILES> InspectCommitEvm for Evm<CTX, INSP, INST, PRECOMPILES>
where
    CTX: ContextSetters
        + ContextTr<Journal: JournalTr<FinalOutput = JournalOutput> + JournalExt, Db: DatabaseCommit>,
    INSP: Inspector<CTX, EthInterpreter>,
    INST: InstructionProvider<Context = CTX, InterpreterTypes = EthInterpreter>,
    PRECOMPILES: PrecompileProvider<CTX, Output = InterpreterResult>,
{
    fn inspect_commit_previous(&mut self) -> Self::CommitOutput {
        self.inspect_replay().map(|r| {
            self.ctx().db().commit(r.state);
            r.result
        })
    }
}
