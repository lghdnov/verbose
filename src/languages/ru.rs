use crate::register_verbalizer;
use crate::verbality::{VerbalizeError, Verbalizer};

pub struct RussianVerbalizer;

const MAX: u64 = 999_999_999_999_999;

impl Verbalizer for RussianVerbalizer {
    fn code(&self) -> &'static str {
        "ru"
    }
    fn name(&self) -> &'static str {
        "Русский"
    }

    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError> {
        if n > MAX {
            return Err(VerbalizeError::NumberTooLarge(n, MAX));
        }
        if n == 0 {
            return Ok("ноль".to_string());
        }
        Ok(verbalize_number(n))
    }
}

register_verbalizer!(&RussianVerbalizer as &dyn Verbalizer);

fn verbalize_number(n: u64) -> String {
    let mut result = Vec::new();
    let mut remaining = n;
    let mut scale = 0;

    while remaining > 0 {
        let chunk = remaining % 1000;

        if chunk > 0 {
            let chunk_text = verbalize_chunk(chunk, scale == 1);

            if scale > 0 {
                let scale_text = select_scale_form(chunk, scale);
                result.push(format!("{chunk_text} {scale_text}"));
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

fn verbalize_chunk(n: u64, feminine: bool) -> String {
    let hundreds = (n / 100) as usize;
    let tens = ((n / 10) % 10) as usize;
    let units = (n % 10) as usize;

    let mut parts = Vec::new();

    if hundreds > 0 {
        parts.push(HUNDREDS[hundreds]);
    }

    let units_words = if feminine {
        UNITS_FEMININE
    } else {
        UNITS_MASCULINE
    };

    match tens {
        0 => {
            if units > 0 {
                parts.push(units_words[units]);
            }
        }
        1 => {
            parts.push(TEENS[units]);
        }
        _ => {
            if tens > 1 {
                parts.push(TENS[tens]);
            }
            if units > 0 {
                parts.push(units_words[units]);
            }
        }
    }

    parts.join(" ")
}

fn select_scale_form(n: u64, scale_idx: usize) -> &'static str {
    let (one, few, many) = SCALES[scale_idx];

    if n % 10 == 1 && n % 100 != 11 {
        one
    } else if (2..=4).contains(&(n % 10)) && !(12..=14).contains(&(n % 100)) {
        few
    } else {
        many
    }
}

const UNITS_MASCULINE: &[&str] = &[
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

const UNITS_FEMININE: &[&str] = &[
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
