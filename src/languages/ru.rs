use crate::verbality::{core::verbalize_number, *};

pub struct RussianVerbalizer;

impl Verbalizer for RussianVerbalizer {
    fn code(&self) -> &'static str {
        "ru"
    }
    fn name(&self) -> &'static str {
        "Русский"
    }
    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError> {
        let mut out = String::with_capacity(128);
        verbalize_number::<RussianVerbalizer, _>(self, n, &mut out)?;
        Ok(out)
    }

    fn zero(&self) -> &'static str {
        "ноль"
    }
    fn chunk_base(&self) -> u64 {
        1000
    }
    fn unit(&self, d: usize, g: Gender) -> &'static str {
        match g {
            Gender::Fem => UNITS_FEM[d],
            _ => UNITS_MASC[d],
        }
    }
    fn teen(&self, d: usize) -> &'static str {
        TEENS[d]
    }
    fn ten(&self, d: usize) -> &'static str {
        TENS[d]
    }
    fn hundred(&self, d: usize) -> &'static str {
        HUNDREDS[d]
    }

    fn scale_form(&self, idx: usize, form: PluralForm) -> &'static str {
        let (one, few, many) = SCALES.get(idx).copied().unwrap_or(("", "", ""));
        match form {
            PluralForm::One => one,
            PluralForm::Few => few,
            PluralForm::Many => many,
        }
    }

    fn plural_for_chunk(&self, chunk: u64, _scale_idx: usize) -> PluralForm {
        const EXCEPTION_START: u64 = 12;
        const EXCEPTION_END: u64 = 14;

        let last_two_digits = chunk % 100;
        let last_digit = chunk % 10;

        let is_singular = last_digit == 1 && last_two_digits != 11;
        let is_few = (2..=4).contains(&last_digit)
            && !(EXCEPTION_START..=EXCEPTION_END).contains(&last_two_digits);

        if is_singular {
            PluralForm::One
        } else if is_few {
            PluralForm::Few
        } else {
            PluralForm::Many
        }
    }

    fn unit_gender_for_scale(&self, scale_idx: usize) -> Gender {
        match scale_idx {
            1 => Gender::Fem,
            _ => Gender::Masc,
        }
    }
}

static RU_VERBALIZER: RussianVerbalizer = RussianVerbalizer;

inventory::submit! {
    &RU_VERBALIZER as &'static dyn crate::verbality::Verbalizer
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
