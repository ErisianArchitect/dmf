
#[derive(Debug, thiserror::Error)]
pub enum Utf8DecodeError {
    #[error("Codepoint is invalid: {0}")]
    InvalidCodepoint(u32),
    #[error("Invalid UTF-8 Encoding.")]
    InvalidUtf8Encoding,
    #[error("Incomplete UTF-8 Sequence.")]
    IncompleteUtf8Sequence,
    #[error("Empty byte sequence.")]
    EmptyByteSequence,
}

pub fn utf8_decode(bytes: &[u8]) -> Result<(char, u32), Utf8DecodeError> {
    if bytes.is_empty() {
        return Err(Utf8DecodeError::EmptyByteSequence);
    }
    let first = bytes[0];
    match first.leading_ones() {
        // 1 byte
        0 => {
            let codepoint = first & 0b01111111;
            let chr = unsafe {
                char::from_u32_unchecked(codepoint as u32)
            };
            Ok((chr, 1))
        }
        // 2 bytes
        2 => {
            if bytes.len() < 2 {
                return Err(Utf8DecodeError::IncompleteUtf8Sequence);
            }
            let mut codepoint = ((first & 0b00011111) as u32) << 6;
            let second = bytes[1];
            if second.leading_ones() == 1 {
                codepoint |= (second & 0b00111111) as u32;
                let Some(chr) = char::from_u32(codepoint) else {
                    return Err(Utf8DecodeError::InvalidCodepoint(codepoint))
                };
                Ok((chr, 2))
            } else {
                Err(Utf8DecodeError::InvalidUtf8Encoding)
            }
        }
        // 3 bytes
        3 => {
            if bytes.len() < 3 {
                return Err(Utf8DecodeError::IncompleteUtf8Sequence);
            }
            let mut codepoint = ((first & 0b00001111) as u32) << 6;
            let second = bytes[1];
            if second.leading_ones() == 1 {
                codepoint = (codepoint | (second & 0b00111111) as u32) << 6;
            } else {
                return Err(Utf8DecodeError::InvalidUtf8Encoding);
            }
            let third = bytes[2];
            if third.leading_ones() == 1 {
                codepoint = codepoint | (third & 0b00111111) as u32;
            } else {
                return Err(Utf8DecodeError::InvalidUtf8Encoding);
            }
            let Some(chr) = char::from_u32(codepoint) else {
                return Err(Utf8DecodeError::InvalidCodepoint(codepoint))
            };
            Ok((chr, 3))
        }
        // 4 bytes
        4 => {
            if bytes.len() < 4 {
                return Err(Utf8DecodeError::IncompleteUtf8Sequence);
            }
            let mut codepoint = ((first & 0b00000111) as u32) << 6;
            let second = bytes[1];
            if second.leading_ones() == 1 {
                codepoint = (codepoint | (second & 0b00111111) as u32) << 6;
            } else {
                return Err(Utf8DecodeError::InvalidUtf8Encoding);
            }
            let third = bytes[2];
            if third.leading_ones() == 1 {
                codepoint = (codepoint | (third & 0b00111111) as u32) << 6;
            } else {
                return Err(Utf8DecodeError::InvalidUtf8Encoding);
            }
            let fourth = bytes[3];
            if fourth.leading_ones() == 1 {
                codepoint = codepoint | (fourth & 0b00111111) as u32;
            } else {
                return Err(Utf8DecodeError::InvalidUtf8Encoding);
            }
            let Some(chr) = char::from_u32(codepoint) else {
                return Err(Utf8DecodeError::InvalidCodepoint(codepoint))
            };
            Ok((chr, 4))
        }
        _ => Err(Utf8DecodeError::InvalidUtf8Encoding),
    }
}


pub fn main() {
    println!("{}", include_str!("test_string.txt"));
    let test_utf8 = "A¢€𐍈";
    // let test_utf8 = "A¢€";
    let mut offset = 0usize;
    let mut chars = test_utf8.chars();
    while offset < test_utf8.len() {
        let (chr, len) = match utf8_decode(test_utf8[offset..].as_bytes()) {
            Ok(inner) => inner,
            Err(err) => panic!("Error: {err:?}"),
        };
        assert_eq!(Some(chr), chars.next());
        offset += len as usize;
    }
    assert_eq!(offset, test_utf8.len());
}