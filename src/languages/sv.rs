use crate::register_verbalizer;
use crate::verbality::{VerbalizeError, Verbalizer};

pub struct SwedishVerbalizer;

const MAX: u64 = 999_999_999_999_999_999;

impl Verbalizer for SwedishVerbalizer {
    fn code(&self) -> &'static str {
        "sv"
    }
    fn name(&self) -> &'static str {
        "Svenska"
    }

    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError> {
        if n > MAX {
            return Err(VerbalizeError::NumberTooLarge(n, MAX));
        }
        if n == 0 {
            return Ok("noll".to_string());
        }
        Ok(verbalize_number(n))
    }
}

register_verbalizer!(&SwedishVerbalizer as &dyn Verbalizer);

fn verbalize_number(n: u64) -> String {
    let mut result = Vec::new();
    let mut remaining = n;
    let mut scale = 0;

    while remaining > 0 {
        let chunk = remaining % 1000;

        if chunk > 0 {
            let chunk_text = verbalize_chunk_with_thousand(chunk, scale);

            if scale > 0 {
                let scale_word = SCALES[scale];
                result.push(format!("{chunk_text} {scale_word}"));
            } else {
                result.push(chunk_text);
            }
        }

        remaining /= 1000;
        scale += 1;
    }

    result.reverse();
    result.join(" ")
}

fn verbalize_chunk_with_thousand(n: u64, scale: usize) -> String {
    if scale == 1 && n == 1 {
        return "ett".to_string();
    }
    verbalize_chunk(n)
}

fn verbalize_chunk(n: u64) -> String {
    let hundreds = n / 100;
    let tens = (n / 10) % 10;
    let units = n % 10;

    let mut parts: Vec<&str> = Vec::new();

    if hundreds > 0 {
        if hundreds == 1 {
            parts.push("ett");
        }
        parts.push(HUNDREDS[hundreds as usize]);
    }

    match tens {
        0 => {
            if units > 0 {
                parts.push(UNITS[units as usize]);
            }
        }
        1 => {
            parts.push(TEENS[units as usize]);
        }
        _ => {
            if tens > 1 {
                parts.push(TENS[tens as usize]);
            }
            if units > 0 {
                parts.push(UNITS[units as usize]);
            }
        }
    }

    parts.join(" ")
}

const UNITS: &[&str] = &[
    "", "en", "två", "tre", "fyra", "fem", "sex", "sju", "åtta", "nio",
];

const TEENS: &[&str] = &[
    "tio", "elva", "tolv", "tretton", "fjorton", "femton", "sexton", "sjutton", "arton", "nitton",
];

const TENS: &[&str] = &[
    "", "", "tjugo", "trettio", "fyrtio", "femtio", "sextio", "sjuttio", "åttio", "hundra",
];

const HUNDREDS: &[&str] = &[
    "",
    "hundra",
    "tvåhundra",
    "trehundra",
    "fyrahundra",
    "femhundra",
    "sexhundra",
    "sjuhundra",
    "åttahundra",
    "niohundra",
];

const SCALES: &[&str] = &["", "tusen", "miljon", "miljard", "biljon", "biljard"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let v = SwedishVerbalizer;
        assert_eq!(v.verbalize(0).unwrap(), "noll");
        assert_eq!(v.verbalize(1).unwrap(), "en");
        assert_eq!(v.verbalize(5).unwrap(), "fem");
        assert_eq!(v.verbalize(21).unwrap(), "tjugo en");
        assert_eq!(v.verbalize(99).unwrap(), "hundra nio");
    }

    #[test]
    fn thousands() {
        let v = SwedishVerbalizer;
        assert_eq!(v.verbalize(1000).unwrap(), "ett tusen");
        assert_eq!(v.verbalize(2000).unwrap(), "två tusen");
    }

    #[test]
    fn millions() {
        let v = SwedishVerbalizer;
        assert_eq!(v.verbalize(1_000_000).unwrap(), "en miljon");
    }
}
