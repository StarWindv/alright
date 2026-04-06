mod modules;

pub use alright_derive::Exception;
pub mod traits {
    pub use crate::modules::{
        traits::{
            template_display::TemplateDisplay,
            exception_utils::ExceptionUtils,
            transform::Transform,
            commonly::{
                PromiseErr,
                AlrightBox,
                AlrightError
            }
        }
    };
}
pub mod commonly {
    pub use crate::modules::commonly::{
        Exception,
        AbstractException,
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
pub use {
    modules::{
        types::{
            property::Property,
            erased_property::ErasedProperty,
            exception::BaseException,
            erased_exception::ErasedBaseException,
            // train::Train,
        },
    }
};
pub(crate) use {
    modules::{
        types::{
            train::Train,
        },
    }
};

extern crate self as alright;

#[macro_export]
macro_rules! exceptions {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(::alright::Exception, Debug, Clone, ::serde::Serialize)]
            pub struct $name {
                pub property: Box<Property<$name>>,
            }
        )*
    };
}
