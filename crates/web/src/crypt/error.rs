use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    // Key
    KeyFailHmac,

    // -- Pwd
    PwdNotMatching,

    // -- Token
    TokenInvalidFormat,
    TokenCannotDecodeIdent,
    TokenCannotDecodeExp,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
