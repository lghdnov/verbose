use crate::register_verbalizer;
use crate::verbality::{VerbalizeError, Verbalizer};

pub struct EnglishVerbalizer;

const MAX: u64 = 999_999_999_999_999_999;

impl Verbalizer for EnglishVerbalizer {
    fn code(&self) -> &'static str {
        "en"
    }
    fn name(&self) -> &'static str {
        "English"
    }

    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError> {
        if n > MAX {
            return Err(VerbalizeError::NumberTooLarge(n, MAX));
        }
        if n == 0 {
            return Ok("zero".to_string());
        }
        Ok(verbalize_number(n))
    }
}

register_verbalizer!(&EnglishVerbalizer as &dyn Verbalizer);

fn verbalize_number(n: u64) -> String {
    let mut result = Vec::new();
    let mut remaining = n;
    let mut scale = 0;

    while remaining > 0 {
        let chunk = remaining % 1000;

        if chunk > 0 {
            let chunk_text = verbalize_chunk(chunk);

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

fn verbalize_chunk(n: u64) -> String {
    let hundreds = (n / 100) as usize;
    let tens = ((n / 10) % 10) as usize;
    let units = (n % 10) as usize;

    let mut parts: Vec<&str> = Vec::new();

    if hundreds > 0 {
        parts.push(HUNDRED_WITH[hundreds]);
    }

    match tens {
        0 => {
            if units > 0 {
                parts.push(UNITS[units]);
            }
        }
        1 => {
            parts.push(UNITS[(n % 100) as usize]);
        }
        _ => {
            if tens > 1 {
                parts.push(TENS[tens]);
            }
            if units > 0 {
                parts.push(UNITS[units]);
            }
        }
    }

    parts.join(" ")
}

const UNITS: &[&str] = &[
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: &[&str] = &[
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: &[&str] = &[
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
];

const HUNDRED_WITH: &[&str] = &[
    "",
    "one hundred",
    "two hundred",
    "three hundred",
    "four hundred",
    "five hundred",
    "six hundred",
    "seven hundred",
    "eight hundred",
    "nine hundred",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(0).unwrap(), "zero");
        assert_eq!(v.verbalize(1).unwrap(), "one");
        assert_eq!(v.verbalize(21).unwrap(), "twenty one");
    }

    #[test]
    fn hundreds() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(100).unwrap(), "one hundred");
        assert_eq!(v.verbalize(256).unwrap(), "two hundred fifty six");
    }

    #[test]
    fn millions() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(1_000_000).unwrap(), "one million");
    }
}
