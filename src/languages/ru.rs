use crate::register_verbalizer;
use crate::verbality::{VerbalizeError, Verbalizer};
use core::fmt;
use std::fmt::Write;

pub struct RussianVerbalizer;

const BASE: u64 = 10;
const DELIMITER: u64 = 100;
const CHUNK_DELIMITER: u64 = 1000;
const MAX_NUMBER: u64 = 999_999_999_999_999;
const FEMININE_SCALE: usize = 1;

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

        let mut out = String::with_capacity(128);
        verbalize_number(n, &mut out)?;
        Ok(out)
    }
}

pub fn verbalize_number<W: Write>(n: u64, out: &mut W) -> Result<(), VerbalizeError> {
    let mut max_scale = 0;
    let mut temp = n;
    while temp >= CHUNK_DELIMITER {
        temp /= CHUNK_DELIMITER;
        max_scale += 1;
    }

    let mut divisor = CHUNK_DELIMITER.pow(max_scale as u32);
    let mut first_chunk = true;

    for scale_idx in (0..=max_scale).rev() {
        let chunk = (n / divisor) % CHUNK_DELIMITER;
        divisor /= CHUNK_DELIMITER;

        if chunk == 0 {
            continue;
        }

        if !first_chunk {
            out.write_char(' ')?;
        }
        first_chunk = false;

        let is_feminine = scale_idx == FEMININE_SCALE;
        verbalize_chunk(chunk, is_feminine, out)?;

        if scale_idx > 0 {
            out.write_char(' ')?;
            out.write_str(select_scale_form(chunk, scale_idx))?;
        }
    }

    Ok(())
}

fn verbalize_chunk<W: Write>(n: u64, feminine: bool, out: &mut W) -> Result<(), VerbalizeError> {
    let hundreds = (n / DELIMITER) as usize;
    let tens = ((n / BASE) % BASE) as usize;
    let units = (n % BASE) as usize;

    let mut needs_space = false;
    let mut write_word = |word: &str| -> Result<(), VerbalizeError> {
        if needs_space {
            out.write_char(' ')?;
        }
        out.write_str(word)?;
        needs_space = true;
        Ok(())
    };

    if hundreds > 0 {
        write_word(HUNDREDS[hundreds])?;
    }

    if tens == 1 {
        write_word(TEENS[units])?;
    } else if tens >= 2 {
        write_word(TENS[tens])?;
    }

    if units > 0 && tens != 1 {
        write_word(if feminine {
            UNITS_FEM[units]
        } else {
            UNITS_MASC[units]
        })?;
    }

    Ok(())
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
