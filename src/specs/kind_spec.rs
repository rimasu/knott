use std::convert::{TryFrom, TryInto};
use crate::specs::suffix_spec::{convert_suffixes, SuffixSpec};
use crate::lookup::{Indexed, Labelled};
use crate::defs::KindDef;
use crate::error::ItemError;
use crate::coords::{Kind, InvalidKind};

#[derive(Debug, PartialEq)]
pub struct KindSpec {
    pub label: String,
    pub id: Kind,
    pub suffixes: SuffixSpec,
}

impl Indexed for KindSpec {
    fn as_usize(&self) -> usize {
        self.id.as_usize()
    }
}

impl Labelled for KindSpec {
    fn label(&self) -> &str {
        &self.label
    }
}

impl From<InvalidKind> for ItemError {
    fn from(e: InvalidKind) -> Self {
        ItemError::InvalidId(e.0)
    }
}


impl TryFrom<KindDef> for KindSpec {
    type Error = ItemError;

    fn try_from(def: KindDef) -> Result<Self, Self::Error> {
        let id: Kind = def.id.try_into()?;
        let suffixes = convert_suffixes(def.suffix_range, def.suffixes)?;

        Ok(KindSpec {
            label: def.label.to_owned(),
            id: id.to_owned(),
            suffixes,
        })
    }
}
