use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

pub mod core;

#[derive(Debug, Clone, PartialEq)]
pub enum VerbalizeError {
    NumberTooLarge(u64, u64),
    Fmt(std::fmt::Error),
}

impl From<std::fmt::Error> for VerbalizeError {
    fn from(err: std::fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

impl fmt::Display for VerbalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerbalizeError::NumberTooLarge(n, m) => {
                write!(f, "Number {} exceeds maximum supported value {}", n, m)
            }
            VerbalizeError::Fmt(err) => write!(f, "Fmt error: {}", err),
        }
    }
}

impl std::error::Error for VerbalizeError {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gender {
    Masc,
    Fem,
    Neut,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PluralForm {
    One,
    Few,
    Many,
}

pub trait Verbalizer: Send + Sync {
    fn code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError>;

    fn zero(&self) -> &'static str;
    fn chunk_base(&self) -> u64;
    fn unit(&self, digit: usize, gender: Gender) -> &'static str;
    fn teen(&self, digit: usize) -> &'static str;
    fn ten(&self, digit: usize) -> &'static str;
    fn hundred(&self, digit: usize) -> &'static str;

    fn scale_form(&self, scale_idx: usize, form: PluralForm) -> &'static str;
    fn plural_for_chunk(&self, chunk: u64, scale_idx: usize) -> PluralForm;
    fn unit_gender_for_scale(&self, scale_idx: usize) -> Gender;
}

inventory::collect!(&'static dyn Verbalizer);

pub struct VerbalizerRegistry {
    map: HashMap<&'static str, &'static dyn Verbalizer>,
}

impl VerbalizerRegistry {
    fn new() -> Self {
        let mut map = HashMap::new();

        for v in inventory::iter::<&'static dyn Verbalizer> {
            map.insert(v.code(), *v);
        }

        Self { map }
    }

    pub fn get(&self, code: &str) -> Option<&dyn Verbalizer> {
        self.map.get(code).copied()
    }

    pub fn codes(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.map.keys().copied()
    }

    pub fn codes_string(&self) -> String {
        self.codes().collect::<Vec<_>>().join(", ")
    }
}

static REGISTRY: LazyLock<VerbalizerRegistry> = LazyLock::new(VerbalizerRegistry::new);

pub fn registry() -> &'static VerbalizerRegistry {
    &REGISTRY
}

#[macro_export]
macro_rules! register_verbalizer {
    ($v:expr) => {
        inventory::submit! {
            &$v as &'static dyn $crate::verbality::Verbalizer
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbalize_error_fmt() {
        let err = std::fmt::Error;
        let e = VerbalizeError::Fmt(err);
        assert!(format!("{}", e).starts_with("Fmt error:"));
    }

    #[test]
    fn registry_get() {
        let r = registry();
        assert!(r.get("ru").is_some());
        assert!(r.get("nonexistent").is_none());
    }

    #[test]
    fn registry_codes() {
        let r = registry();
        let codes: Vec<_> = r.codes().collect();
        assert!(!codes.is_empty());
    }

    #[test]
    fn registry_codes_string() {
        let r = registry();
        let s = r.codes_string();
        assert!(!s.is_empty());
    }
}
