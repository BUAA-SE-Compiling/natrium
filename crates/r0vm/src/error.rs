pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    InvalidInstruction,
    StackOverflow,
    StackUnderflow,
    AttemptToReadEmptyStack,
    InvalidAddress,
    DivZero,
    OutOfMemory,
}
