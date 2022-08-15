//! #Errors encountered when using resrs-lang

/**
 * Kinds of errors encoutered when compiling
 */
#[derive(Debug, PartialEq, Eq)]
pub enum CompilationErrKind {
    /**
     * Number Lexed isn't valid.
     * Valid numbers include: 1, 1., .1
     */
    InvalidNumber, // Number

    /**
     * This will probably be unreachable but it's good to have in case we will ever need it
     */
    InvalidString,

    /** Code should not be read */
    Unreachable,
}

impl CompilationErrKind {
    /// Returns `true` if the compilation err kind is [`InvalidString`].
    ///
    /// [`InvalidString`]: CompilationErrKind::InvalidString
    #[must_use]
    pub fn is_invalid_string(&self) -> bool {
        matches!(self, Self::InvalidString)
    }

    /// Returns `true` if the compilation err kind is [`InvalidNumber`].
    ///
    /// [`InvalidNumber`]: CompilationErrKind::InvalidNumber
    #[must_use]
    pub fn is_invalid_number(&self) -> bool {
        matches!(self, Self::InvalidNumber)
    }

    /// Returns `true` if the compilation err kind is [`Unreachable`].
    ///
    /// [`Unreachable`]: CompilationErrKind::Unreachable
    #[must_use]
    pub fn is_unreachable(&self) -> bool {
        matches!(self, Self::Unreachable)
    }
}

/**
 * Thrown during compilation
 *
 *
 */
#[derive(Debug, PartialEq, Eq)]
pub struct CompilationErr {
    /** What kind of error gets thrown */
    pub kind: CompilationErrKind,

    /** Message associated with error thrown */
    pub message: String,
}
