use crate::common::print_annot;
use crate::common::Annot;
use crate::common::Loc;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    TapeBufferOverflow,
    NegativePosition,
    CannotDecodeCharacter,
    CannotReadCharacter,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl InterpreterError {
    pub fn buffer_overflow(loc: Loc) -> Self {
        Self::new(InterpreterErrorKind::TapeBufferOverflow, loc)
    }
    pub fn negative_postion(loc: Loc) -> Self {
        Self::new(InterpreterErrorKind::NegativePosition, loc)
    }
    pub fn cannot_decode_character(loc: Loc) -> Self {
        Self::new(InterpreterErrorKind::CannotDecodeCharacter, loc)
    }
    pub fn cannot_read_character(loc: Loc) -> Self {
        Self::new(InterpreterErrorKind::CannotReadCharacter, loc)
    }
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InterpreterErrorKind::*;
        match self.value {
            TapeBufferOverflow => write!(f, "tape buffer over flow."),
            NegativePosition => write!(f, "tape negative position."),
            CannotDecodeCharacter => write!(f, "can not decode."),
            CannotReadCharacter => write!(f, "can not read."),
        }
    }
}

impl StdError for InterpreterError {
    fn description(&self) -> &str {
        use self::InterpreterErrorKind::*;
        match self.value {
            TapeBufferOverflow => "tape buffer over flow.",
            NegativePosition => "tape negative position.",
            CannotDecodeCharacter => "can not decode.",
            CannotReadCharacter => "can not read.",
        }
    }
}

impl InterpreterError {
    pub fn show_diagnostic(&self, input: &str) {
        // エラー情報を簡単に表示し
        eprintln!("{}", self);
        // エラー位置を指示する
        print_annot(input, self.loc.clone());
    }
}
