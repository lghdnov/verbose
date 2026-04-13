use crate::register_verbalizer;
use crate::verbality::{VerbalizeError, Verbalizer};

pub struct RussianVerbalizer;

const BASE: u64 = 10;
const DELIMITER: u64 = 100;
const CHUNK_DELIMITER: u64 = 1000;
const MAX_NUMBER: u64 = 999_999_999_999_999;
const FEMININE_SCALE: usize = 1;
const SCALE_COUNT: usize = 5;

impl Verbalizer for RussianVerbalizer {
    fn code(&self) -> &'static str {
        "ru"
    }
    fn name(&self) -> &'static str {
        "Русский"
    }

    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError> {
        if n > MAX_NUMBER {
            return Err(VerbalizeError::NumberTooLarge(n, MAX_NUMBER));
        }
        if n == 0 {
            return Ok("ноль".to_string());
        }
        Ok(verbalize_number(n))
    }
}

register_verbalizer!(&RussianVerbalizer as &dyn Verbalizer);

fn verbalize_number(n: u64) -> String {
    let mut parts = Vec::with_capacity(SCALE_COUNT);
    let mut remaining = n;
    let mut scale_idx = 0;

    while remaining > 0 {
        let chunk = remaining % CHUNK_DELIMITER;
        remaining /= CHUNK_DELIMITER;

        if chunk == 0 {
            scale_idx += 1;
            continue;
        }

        let is_feminine = scale_idx == FEMININE_SCALE;
        let chunk_text = verbalize_chunk(chunk, is_feminine);
        let part = match scale_idx {
            0 => chunk_text,
            _ => {
                let scale = select_scale_form(chunk, scale_idx);
                format!("{chunk_text} {scale}")
            }
        };

        parts.push(part);
        scale_idx += 1;
    }

    parts.reverse();
    parts.join(" ")
}

fn verbalize_chunk(n: u64, feminine: bool) -> String {
    let hundreds = (n / DELIMITER) as usize;
    let tens = ((n / BASE) % BASE) as usize;
    let units = (n % BASE) as usize;

    let mut words = Vec::with_capacity(3);
    if hundreds > 0 {
        words.push(HUNDREDS[hundreds]);
    }

    match tens {
        1 => words.push(TEENS[units]),
        2..=9 => words.push(TENS[tens]),
        _ => {}
    }

    if units > 0 && tens != 1 {
        let unit_word = if feminine {
            UNITS_FEM[units]
        } else {
            UNITS_MASC[units]
        };
        words.push(unit_word);
    }

    words.join(" ")
}

fn select_scale_form(n: u64, scale_idx: usize) -> &'static str {
    let (one, few, many) = SCALES[scale_idx];
    let last_digit = n % BASE;
    let last_two = n % DELIMITER;

    if last_digit == 1 && last_two != 11 {
        one
    } else if (2..=4).contains(&last_digit) && !(12..=14).contains(&last_two) {
        few
    } else {
        many
    }
}

const UNITS_MASC: &[&str] = &[
    "",
    "один",
    "два",
    "три",
    "четыре",
    "пять",
    "шесть",
    "семь",
    "восемь",
    "девять",
];

const UNITS_FEM: &[&str] = &[
    "",
    "одна",
    "две",
    "три",
    "четыре",
    "пять",
    "шесть",
    "семь",
    "восемь",
    "девять",
];

const TEENS: &[&str] = &[
    "десять",
    "одиннадцать",
    "двенадцать",
    "тринадцать",
    "четырнадцать",
    "пятнадцать",
    "шестнадцать",
    "семнадцать",
    "восемнадцать",
    "девятнадцать",
];

const TENS: &[&str] = &[
    "",
    "",
    "двадцать",
    "тридцать",
    "сорок",
    "пятьдесят",
    "шестьдесят",
    "семьдесят",
    "восемьдесят",
    "девяносто",
];

const HUNDREDS: &[&str] = &[
    "",
    "сто",
    "двести",
    "триста",
    "четыреста",
    "пятьсот",
    "шестьсот",
    "семьсот",
    "восемьсот",
    "девятьсот",
];

const SCALES: &[(&str, &str, &str)] = &[
    ("", "", ""),
    ("тысяча", "тысячи", "тысяч"),
    ("миллион", "миллиона", "миллионов"),
    ("миллиард", "миллиарда", "миллиардов"),
    ("триллион", "триллиона", "триллионов"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let v = RussianVerbalizer;
        assert_eq!(v.verbalize(0).unwrap(), "ноль");
        assert_eq!(v.verbalize(1).unwrap(), "один");
        assert_eq!(v.verbalize(5).unwrap(), "пять");
        assert_eq!(v.verbalize(10).unwrap(), "десять");
        assert_eq!(v.verbalize(21).unwrap(), "двадцать один");
        assert_eq!(v.verbalize(99).unwrap(), "девяносто девять");
    }

    #[test]
    fn hundreds() {
        let v = RussianVerbalizer;
        assert_eq!(v.verbalize(100).unwrap(), "сто");
        assert_eq!(v.verbalize(256).unwrap(), "двести пятьдесят шесть");
    }

    #[test]
    fn thousands() {
        let v = RussianVerbalizer;
        assert_eq!(v.verbalize(1000).unwrap(), "одна тысяча");
        assert_eq!(v.verbalize(2000).unwrap(), "две тысячи");
        assert_eq!(v.verbalize(5000).unwrap(), "пять тысяч");
    }

    #[test]
    fn millions() {
        let v = RussianVerbalizer;
        assert_eq!(v.verbalize(1_000_000).unwrap(), "один миллион");
        assert_eq!(v.verbalize(2_000_000).unwrap(), "два миллиона");
    }
}
