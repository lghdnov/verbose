use crate::verbality::Verbalizer;

pub struct RussianVerbalizer;

impl Verbalizer for RussianVerbalizer {
    fn code(&self) -> &'static str {
        "ru"
    }

    fn name(&self) -> &'static str {
        "Русский"
    }

    fn verbalize(&self, n: u64) -> String {
        if n == 0 {
            return "ноль".to_string();
        }
        verbalize_internal(n)
    }
}

fn verbalize_internal(n: u64) -> String {
    const UNITS: &[&str] = &[
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
    const UNITS_F: &[&str] = &[
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

    fn verbalize_chunk(n: u64, use_feminine: bool) -> String {
        let units = if use_feminine { UNITS_F } else { UNITS };

        let h = (n / 100) as usize;
        let t = ((n / 10) % 10) as usize;
        let u = (n % 10) as usize;

        let mut parts = Vec::new();

        if h > 0 {
            parts.push(HUNDREDS[h].to_string());
        }

        if t == 1 {
            if u > 0 {
                parts.push(TEENS[u].to_string());
            } else {
                parts.push("десять".to_string());
            }
        } else {
            if t > 1 {
                parts.push(TENS[t].to_string());
            }
            if u > 0 {
                parts.push(units[u].to_string());
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
            let use_feminine = scale_idx == 1;
            let chunk_str = verbalize_chunk(chunk, use_feminine);

            if scale_idx > 0 {
                let scale = match SCALES[scale_idx] {
                    (one, few, many) => {
                        if chunk % 10 == 1 && chunk % 100 != 11 {
                            one
                        } else if (2..=4).contains(&(chunk % 10))
                            && !(12..=14).contains(&(chunk % 100))
                        {
                            few
                        } else {
                            many
                        }
                    }
                };
                if !chunk_str.is_empty() {
                    parts.push(format!("{} {}", chunk_str, scale));
                }
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
mod russian {
    use super::*;

    #[test]
    fn basic_numbers() {
        let v = RussianVerbalizer;

        let tests = [
            (0, "ноль"),
            (1, "один"),
            (2, "два"),
            (3, "три"),
            (4, "четыре"),
            (5, "пять"),
            (10, "десять"),
            (11, "одиннадцать"),
            (19, "девятнадцать"),
            (20, "двадцать"),
            (21, "двадцать один"),
            (30, "тридцать"),
            (45, "сорок пять"),
            (99, "девяносто девять"),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n), expected);
        }
    }

    #[test]
    fn hundreds() {
        let v = RussianVerbalizer;

        let tests = [
            (100, "сто"),
            (101, "сто один"),
            (110, "сто десять"),
            (125, "сто двадцать пять"),
            (200, "двести"),
            (256, "двести пятьдесят шесть"),
            (999, "девятьсот девяносто девять"),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n), expected);
        }
    }

    #[test]
    fn thousands() {
        let v = RussianVerbalizer;

        let tests = [
            (1000, "одна тысяча"),
            (1001, "одна тысяча один"),
            (1010, "одна тысяча десять"),
            (1100, "одна тысяча сто"),
            (2000, "две тысячи"),
            (5000, "пять тысяч"),
            (11000, "одиннадцать тысяч"),
            (21000, "двадцать одна тысяча"),
            (25000, "двадцать пять тысяч"),
            (
                999999,
                "девятьсот девяносто девять тысяч девятьсот девяносто девять",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n), expected);
        }
    }

    #[test]
    fn millions() {
        let v = RussianVerbalizer;

        let tests = [
            (1_000_000, "один миллион"),
            (2_000_000, "два миллиона"),
            (5_000_000, "пять миллионов"),
            (1_100_000, "один миллион сто тысяч"),
            (
                2_345_678,
                "два миллиона триста сорок пять тысяч шестьсот семьдесят восемь",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n), expected);
        }
    }

    #[test]
    fn billions() {
        let v = RussianVerbalizer;

        let tests = [
            (1_000_000_000, "один миллиард"),
            (2_000_000_000, "два миллиарда"),
            (5_000_000_000, "пять миллиардов"),
            (
                1_234_567_890,
                "один миллиард двести тридцать четыре миллиона пятьсот шестьдесят семь тысяч восемьсот девяносто",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n), expected);
        }
    }

    #[test]
    fn trillions() {
        let v = RussianVerbalizer;

        let tests = [
            (1_000_000_000_000, "один триллион"),
            (2_000_000_000_000, "два триллиона"),
            (5_000_000_000_000, "пять триллионов"),
            (
                1_234_000_000_000,
                "один триллион двести тридцать четыре миллиарда",
            ),
        ];

        for (n, expected) in tests {
            assert_eq!(v.verbalize(n), expected);
        }
    }
}
