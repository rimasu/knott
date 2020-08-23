use std::convert::{TryFrom, TryInto};
use crate::specs::kind::{Kind, InvalidKind};
use crate::specs::suffix::{SuffixRange, convert_optional_suffix_range};
use crate::lookup::{Indexed, Labelled};
use crate::defs::KindDef;
use crate::error::ItemError;

#[derive(Debug, PartialEq)]
pub struct KindSpec {
    label: String,
    id: Kind,
    suffix_range: Option<SuffixRange>,
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
        let suffix_range = convert_optional_suffix_range(def.suffix_range)?;

        Ok(KindSpec {
            label: def.label.to_owned(),
            id: id.to_owned(),
            suffix_range,
        })
    }
}
