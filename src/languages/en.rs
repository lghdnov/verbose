use crate::register_verbalizer;
use crate::verbality::{VerbalizeError, Verbalizer};

pub struct EnglishVerbalizer;

const MAX_VERBALIZABLE: u64 = 999_999_999_999_999_999; // 10^18 - 1 (quintillion - 1)

impl Verbalizer for EnglishVerbalizer {
    fn code(&self) -> &'static str {
        "en"
    }

    fn name(&self) -> &'static str {
        "English"
    }

    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError> {
        if n > MAX_VERBALIZABLE {
            return Err(VerbalizeError::NumberTooLarge(n, MAX_VERBALIZABLE));
        }
        if n == 0 {
            return Ok("zero".to_string());
        }
        Ok(verbalize_internal(n))
    }
}

register_verbalizer!(&EnglishVerbalizer as &dyn Verbalizer);

fn verbalize_internal(n: u64) -> String {
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

    fn verbalize_chunk(n: u64) -> String {
        let h = n / 100;
        let t = (n / 10) % 10;
        let u = n % 10;

        let mut parts = Vec::new();

        if h > 0 {
            parts.push(format!("{} hundred", UNITS[h as usize]));
        }

        if t == 1 {
            parts.push(UNITS[(n % 100) as usize].to_string());
        } else {
            if t > 1 {
                parts.push(TENS[t as usize].to_string());
            }
            if u > 0 {
                parts.push(UNITS[u as usize].to_string());
            }
        }

        parts.join(" ")
    }

    let mut parts = Vec::new();
    let mut n = n;
    let mut scale_idx = 0;

    while n > 0 {
        let chunk = n % 1000;
        if chunk > 0 {
            let chunk_str = verbalize_chunk(chunk);
            if scale_idx > 0 {
                parts.push(format!("{} {}", chunk_str, SCALES[scale_idx]));
            } else {
                parts.push(chunk_str);
            }
        }
        n /= 1000;
        scale_idx += 1;
    }

    parts.reverse();
    parts.join(" ")
}

#[cfg(test)]
mod english {
    use super::*;

    #[test]
    fn basic_numbers() {
        let v = EnglishVerbalizer;

        let tests = [
            (0, "zero"),
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (10, "ten"),
            (11, "eleven"),
            (19, "nineteen"),
            (20, "twenty"),
            (21, "twenty one"),
            (45, "forty five"),
            (99, "ninety nine"),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n).unwrap(), expected);
        }
    }

    #[test]
    fn hundreds() {
        let v = EnglishVerbalizer;

        let tests = [
            (100, "one hundred"),
            (101, "one hundred one"),
            (110, "one hundred ten"),
            (125, "one hundred twenty five"),
            (200, "two hundred"),
            (999, "nine hundred ninety nine"),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n).unwrap(), expected);
        }
    }

    #[test]
    fn thousands() {
        let v = EnglishVerbalizer;

        let tests = [
            (1000, "one thousand"),
            (1001, "one thousand one"),
            (1010, "one thousand ten"),
            (1100, "one thousand one hundred"),
            (2000, "two thousand"),
            (25_000, "twenty five thousand"),
            (
                999_999,
                "nine hundred ninety nine thousand nine hundred ninety nine",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n).unwrap(), expected);
        }
    }

    #[test]
    fn millions() {
        let v = EnglishVerbalizer;

        let tests = [
            (1_000_000, "one million"),
            (2_000_000, "two million"),
            (5_000_000, "five million"),
            (
                2_345_678,
                "two million three hundred forty five thousand six hundred seventy eight",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n).unwrap(), expected);
        }
    }

    #[test]
    fn billions() {
        let v = EnglishVerbalizer;

        let tests = [
            (1_000_000_000, "one billion"),
            (2_000_000_000, "two billion"),
            (5_000_000_000, "five billion"),
            (
                1_234_567_890,
                "one billion two hundred thirty four million five hundred sixty seven thousand eight hundred ninety",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n).unwrap(), expected);
        }
    }

    #[test]
    fn trillions() {
        let v = EnglishVerbalizer;

        let tests = [
            (1_000_000_000_000, "one trillion"),
            (2_000_000_000_000, "two trillion"),
            (5_000_000_000_000, "five trillion"),
            (
                1_234_000_000_000,
                "one trillion two hundred thirty four billion",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n).unwrap(), expected);
        }
    }
}
