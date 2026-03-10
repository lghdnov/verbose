use std::collections::HashMap;

pub trait Verbalizer: Send + Sync {
    fn code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn verbalize(&self, n: u64) -> String;
}

pub struct VerbalizerRegistry {
    map: HashMap<&'static str, Box<dyn Verbalizer>>,
}

impl VerbalizerRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[cfg(feature = "lang-ru")]
    pub fn with_ru(self) -> Self {
        use crate::languages::ru::RussianVerbalizer;
        self.with(RussianVerbalizer)
    }

    #[cfg(feature = "lang-en")]
    pub fn with_en(self) -> Self {
        use crate::languages::en::EnglishVerbalizer;
        self.with(EnglishVerbalizer)
    }

    pub fn with<V: Verbalizer + 'static>(mut self, verbalizer: V) -> Self {
        self.map.insert(verbalizer.code(), Box::new(verbalizer));
        self
    }

    pub fn get(&self, code: &str) -> Option<&dyn Verbalizer> {
        self.map.get(code).map(|b| b.as_ref())
    }

    pub fn codes(&self) -> Vec<&'static str> {
        self.map.keys().copied().collect()
    }
}

impl Default for VerbalizerRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        #[cfg(feature = "lang-ru")]
        {
            use crate::languages::ru::RussianVerbalizer;
            registry = registry.with(RussianVerbalizer);
        }
        #[cfg(feature = "lang-en")]
        {
            use crate::languages::en::EnglishVerbalizer;
            registry = registry.with(EnglishVerbalizer);
        }
        registry
    }
}
