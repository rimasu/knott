use std::convert::{TryFrom, TryInto};
use crate::specs::kind::{Kind, InvalidKind};
use crate::specs::suffix::{convert_suffixes, Suffixes};
use crate::lookup::{Indexed, Labelled};
use crate::defs::KindDef;
use crate::error::ItemError;

#[derive(Debug, PartialEq)]
pub struct KindSpec {
    label: String,
    id: Kind,
    suffixes: Suffixes,
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
