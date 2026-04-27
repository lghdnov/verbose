use std::collections::HashMap;
use std::sync::LazyLock;

use crate::verbalizer::error::VerbalizeError;

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
