// alright-common
// github.com/StarWindv/alright.git
// GPL-3.0-Only
#[deprecated(
    since = "0.1.3",
    note = "This package has been merged with `alright` package"
)]
pub use alright::traits::{
    PromiseErr, AlrightBox, AlrightError
};
pub mod commonly_exceptions {
    pub use alright::commonly::{
        AbstractException,
        Exception,
        JustAException,
        KeyboardInterrupt,
        SystemExit,
        ArithmeticError,
        AssertionError,
        AttributeError,
        BufferError,
        EOFError,
        MemoryError,
        NameError,
        ReferenceError,
        RuntimeError,
        StopAsyncIteration,
        StopIteration,
        SystemError,
        TypeError,
        ValueError,
        FloatingPointError,
        OverflowError,
        ZeroDivisionError,
        BlockingIOError,
        ChildProcessError,
        ConnectionError,
        FileExistsError,
        FileNotFoundError,
        InterruptedError,
        IsADirectoryError,
        NotADirectoryError,
        PermissionError,
        ProcessLookupError,
        TimeoutError,
        IndexError,
        KeyError,
        NotImplementedError,
        RecursionError,
        UnicodeError,
        BrokenPipeError,
        ConnectionAbortedError,
        ConnectionRefusedError,
        ConnectionResetError,
        UnicodeDecodeError,
        UnicodeEncodeError,
        UnicodeTranslateError,
        OSError,
    };
}
