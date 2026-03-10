use std::collections::HashMap;
use std::sync::LazyLock;

pub trait Verbalizer: Send + Sync {
    fn code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn verbalize(&self, n: u64) -> String;
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
