// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

// endregion: --- Modules

// region:    --- Time
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap()
}
// endregion: --- Time
