use std::collections::HashMap;
use std::sync::LazyLock;

pub trait Verbalizer: Send + Sync {
    fn code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn verbalize(&self, n: u64) -> String;
}

#[derive(Clone)]
pub struct VerbalizerRegistry {
    map: HashMap<&'static str, &'static dyn Verbalizer>,
}

impl VerbalizerRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, code: &str) -> Option<&dyn Verbalizer> {
        self.map.get(code).copied()
    }

    pub fn codes(&self) -> Vec<&'static str> {
        self.map.keys().copied().collect()
    }
}

static REGISTRY: LazyLock<VerbalizerRegistry> = LazyLock::new(|| {
    let mut registry = VerbalizerRegistry::new();

    #[cfg(feature = "lang-ru")]
    {
        use crate::languages::ru::RussianVerbalizer;
        let v: &'static dyn Verbalizer = &RussianVerbalizer;
        registry.map.insert("ru", v);
    }

    #[cfg(feature = "lang-en")]
    {
        use crate::languages::en::EnglishVerbalizer;
        let v: &'static dyn Verbalizer = &EnglishVerbalizer;
        registry.map.insert("en", v);
    }

    registry
});

impl Default for VerbalizerRegistry {
    fn default() -> Self {
        REGISTRY.clone()
    }
}

impl Default for &VerbalizerRegistry {
    fn default() -> Self {
        &REGISTRY
    }
}
