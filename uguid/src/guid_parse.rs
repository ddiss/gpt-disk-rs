// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Guid;
use core::fmt::{self, Display, Formatter};

/// Error type for [`Guid::try_parse`] and [`Guid::from_str`].
///
/// If the `std` feature is enabled, this type implements the [`Error`]
/// trait.
///
/// [`Error`]: std::error::Error
/// [`Guid::from_str`]: core::str::FromStr::from_str
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GuidFromStrError;

impl Display for GuidFromStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("GUID hex string does not match expected format \"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx\"")
    }
}

/// Parse a hexadecimal ASCII character as a `u8`.
const fn parse_byte_from_ascii_char(c: u8) -> Result<u8, GuidFromStrError> {
    match c {
        b'0' => Ok(0x0),
        b'1' => Ok(0x1),
        b'2' => Ok(0x2),
        b'3' => Ok(0x3),
        b'4' => Ok(0x4),
        b'5' => Ok(0x5),
        b'6' => Ok(0x6),
        b'7' => Ok(0x7),
        b'8' => Ok(0x8),
        b'9' => Ok(0x9),
        b'a' | b'A' => Ok(0xa),
        b'b' | b'B' => Ok(0xb),
        b'c' | b'C' => Ok(0xc),
        b'd' | b'D' => Ok(0xd),
        b'e' | b'E' => Ok(0xe),
        b'f' | b'F' => Ok(0xf),
        _ => Err(GuidFromStrError),
    }
}

/// Macro replacement for the `?` operator, which cannot be used in
/// const functions.
macro_rules! mtry {
    ($expr:expr $(,)?) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        }
    };
}

/// Parse a pair of hexadecimal ASCII characters as a `u8`. For example,
/// `(b'1', b'a')` is parsed as `0x1a`.
const fn parse_byte_from_ascii_char_pair(
    a: u8,
    b: u8,
) -> Result<u8, GuidFromStrError> {
    let a = mtry!(parse_byte_from_ascii_char(a));
    let b = mtry!(parse_byte_from_ascii_char(b));
    Ok(a << 4 | b)
}

/// Parse a pair of hexadecimal ASCII characters at position `start` as
/// a `u8`.
const fn parse_byte_from_ascii_str_at(
    s: &[u8],
    start: usize,
) -> Result<u8, GuidFromStrError> {
    parse_byte_from_ascii_char_pair(s[start], s[start + 1])
}

pub(crate) const fn try_parse_guid(s: &str) -> Result<Guid, GuidFromStrError> {
    // Treat input as ASCII.
    let s = s.as_bytes();

    if s.len() != 36 {
        return Err(GuidFromStrError);
    }

    let sep = b'-';
    if s[8] != sep || s[13] != sep || s[18] != sep || s[23] != sep {
        return Err(GuidFromStrError);
    }

    Ok(Guid {
        time_low: [
            mtry!(parse_byte_from_ascii_str_at(s, 6)),
            mtry!(parse_byte_from_ascii_str_at(s, 4)),
            mtry!(parse_byte_from_ascii_str_at(s, 2)),
            mtry!(parse_byte_from_ascii_str_at(s, 0)),
        ],
        time_mid: [
            mtry!(parse_byte_from_ascii_str_at(s, 11)),
            mtry!(parse_byte_from_ascii_str_at(s, 9)),
        ],
        time_high_and_version: [
            mtry!(parse_byte_from_ascii_str_at(s, 16)),
            mtry!(parse_byte_from_ascii_str_at(s, 14)),
        ],
        clock_seq_high_and_reserved: mtry!(parse_byte_from_ascii_str_at(s, 19)),
        clock_seq_low: mtry!(parse_byte_from_ascii_str_at(s, 21)),
        node: [
            mtry!(parse_byte_from_ascii_str_at(s, 24)),
            mtry!(parse_byte_from_ascii_str_at(s, 26)),
            mtry!(parse_byte_from_ascii_str_at(s, 28)),
            mtry!(parse_byte_from_ascii_str_at(s, 30)),
            mtry!(parse_byte_from_ascii_str_at(s, 32)),
            mtry!(parse_byte_from_ascii_str_at(s, 34)),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_byte_from_ascii_char_pair(b'1', b'a'), Ok(0x1a));
        assert_eq!(parse_byte_from_ascii_char_pair(b'8', b'f'), Ok(0x8f));
    }
}
