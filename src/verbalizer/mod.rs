pub mod core;
pub mod error;
pub mod types;

pub use core::verbalize_number;
pub use error::VerbalizeError;
pub use types::{registry, Gender, PluralForm, Verbalizer, VerbalizerBackend, VerbalizerRegistry};

#[macro_export]
macro_rules! register_verbalizer {
    ($v:expr) => {
        inventory::submit! {
            &$v as &'static dyn $crate::verbalizer::Verbalizer
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
