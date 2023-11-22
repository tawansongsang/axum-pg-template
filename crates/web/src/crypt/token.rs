use crate::config;
use crate::crypt::{Error, Result};

// region:    --- Token Type

/// String format: `ident_b64u.ext_b64u.sign_b64u`
pub struct Token {
    pub ident: String,     // Identifier (username for example).
    pub exp: String,       // Expiration date in Rfc3339.
    pub sign_b64u: String, // Signature, base64url encode.
}

// FIXME: FromStr

// FIXME: Display

// endregion: --- Token Type

// region:    --- Web Token Gen and Validation

// endregion: --- Web Token Gen and Validation

// region:    --- (private) Token Gen and Validation

// endregion: --- (private) Token Gen and Validation
