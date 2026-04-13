use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub enum VerbalizeError {
    NumberTooLarge(u64, u64),
    FmtError(std::fmt::Error),
}

impl From<std::fmt::Error> for VerbalizeError {
    fn from(err: std::fmt::Error) -> Self {
        Self::FmtError(err)
    }
}

impl fmt::Display for VerbalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerbalizeError::NumberTooLarge(n, m) => {
                write!(f, "Number {} exceeds maximum supported value {}", n, m)
            }
            VerbalizeError::FmtError(err) => write!(f, "Fmt error: {}", err),
        }
    }
}

impl std::error::Error for VerbalizeError {}

pub trait Verbalizer: Send + Sync {
    fn code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn verbalize(&self, n: u64) -> Result<String, VerbalizeError>;
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
        inventory::submit!($v as &'static dyn $crate::verbality::Verbalizer);
    };
}
