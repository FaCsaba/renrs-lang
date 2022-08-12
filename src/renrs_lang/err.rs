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

    /** Code should not be read */
    Unreachable,
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
