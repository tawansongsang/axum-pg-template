use std::fmt::Display;

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
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// endregion: --- Token Type

// region:    --- Web Token Gen and Validation
pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &str, salt: &str) -> Result<()> {
    let config = &config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

// endregion: --- Web Token Gen and Validation

// region:    --- (private) Token Gen and Validation
fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    todo!()
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    todo!()
}

/// Create toekn signature from token parts
/// and salt.
fn _token_sign_into_b64u(ident: &str, exp: &str, salt: &str, key: &[u8]) -> Result<String> {
    todo!()
}
// endregion: --- (private) Token Gen and Validation

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_token_display_ok() -> Result<()> {
        // -- Fixtures
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2023-11-23T10:00:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        // -- Exec
        println!("->> {fx_token}");

        Ok(())
    }
}
// endregion: --- Tests
