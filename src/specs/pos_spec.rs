use std::convert::{TryFrom, TryInto};
use crate::specs::pos::{Pos, InvalidPos};
use crate::specs::suffix::{convert_suffixes, Suffixes};
use crate::lookup::{Indexed, Labelled};
use crate::defs::PosDef;
use crate::error::ItemError;

#[derive(Debug, PartialEq)]
pub struct PosSpec {
    label: String,
    id: Pos,
    suffixes: Suffixes,
    separate: bool,
    ordered: bool,
    hidden: bool,
}

impl Indexed for PosSpec {
    fn as_usize(&self) -> usize {
        self.id.as_usize()
    }
}

impl Labelled for PosSpec {
    fn label(&self) -> &str {
        &self.label
    }
}

impl From<InvalidPos> for ItemError {
    fn from(e: InvalidPos) -> Self {
        ItemError::InvalidId(e.0)
    }
}

impl TryFrom<PosDef> for PosSpec {
    type Error = ItemError;

    fn try_from(def: PosDef) -> Result<Self, Self::Error> {
        let id: Pos = def.id.try_into()?;
        let suffixes = convert_suffixes(def.suffix_range, def.suffixes)?;

        Ok(PosSpec {
            label: def.label.to_owned(),
            id: id.to_owned(),
            suffixes,
            separate: def.separate,
            ordered: def.ordered,
            hidden: def.hidden,
        })
    }
}