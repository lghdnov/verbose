use crate::{
    register_verbalizer,
    verbality::{core::verbalize_number, *},
};

pub struct EnglishVerbalizer;

impl Verbalizer for EnglishVerbalizer {
    fn code(&self) -> &'static str {
        "en"
    }
    fn name(&self) -> &'static str {
        "English"
    }
    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError> {
        let mut out = String::with_capacity(128);
        verbalize_number::<EnglishVerbalizer, _>(self, n, &mut out)?;
        Ok(out)
    }

    fn zero(&self) -> &'static str {
        "zero"
    }
    fn chunk_base(&self) -> u64 {
        1000
    }
    fn unit(&self, d: usize, _g: Gender) -> &'static str {
        UNITS[d]
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

    fn plural_for_chunk(&self, chunk: u64, scale_idx: usize) -> PluralForm {
        if scale_idx == 0 {
            return PluralForm::Many;
        }
        match chunk {
            1 => PluralForm::One,
            _ => PluralForm::Many,
        }
    }

    fn unit_gender_for_scale(&self, _scale_idx: usize) -> Gender {
        Gender::Masc
    }
}

static EN_VERBALIZER: EnglishVerbalizer = EnglishVerbalizer;
register_verbalizer!(EN_VERBALIZER);

const UNITS: &[&str] = &[
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: &[&str] = &[
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

const HUNDREDS: &[&str] = &[
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

const SCALES: &[(&str, &str, &str)] = &[
    ("", "", ""),
    ("thousand", "thousand", "thousand"),
    ("million", "million", "million"),
    ("billion", "billion", "billion"),
    ("trillion", "trillion", "trillion"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(0).unwrap(), "zero");
        assert_eq!(v.verbalize(1).unwrap(), "one");
        assert_eq!(v.verbalize(5).unwrap(), "five");
        assert_eq!(v.verbalize(10).unwrap(), "ten");
        assert_eq!(v.verbalize(21).unwrap(), "twenty one");
        assert_eq!(v.verbalize(99).unwrap(), "ninety nine");
    }

    #[test]
    fn hundreds() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(100).unwrap(), "one hundred");
        assert_eq!(v.verbalize(256).unwrap(), "two hundred fifty six");
    }

    #[test]
    fn thousands() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(1000).unwrap(), "one thousand");
        assert_eq!(v.verbalize(2000).unwrap(), "two thousand");
        assert_eq!(v.verbalize(5000).unwrap(), "five thousand");
    }

    #[test]
    fn millions() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(1_000_000).unwrap(), "one million");
        assert_eq!(v.verbalize(2_000_000).unwrap(), "two million");
    }

    #[test]
    fn large_numbers() {
        let v = EnglishVerbalizer;
        assert_eq!(
            v.verbalize(999_000_000_000_000).unwrap(),
            "nine hundred ninety nine trillion"
        );
        assert_eq!(v.verbalize(1_000_000_001).unwrap(), "one billion one");
    }

    #[test]
    fn edge_cases() {
        let v = EnglishVerbalizer;
        assert_eq!(v.verbalize(200).unwrap(), "two hundred");
        assert_eq!(v.verbalize(101).unwrap(), "one hundred one");
        assert_eq!(v.verbalize(111).unwrap(), "one hundred eleven");
        assert_eq!(v.verbalize(1001).unwrap(), "one thousand one");
    }
}
