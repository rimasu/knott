use std::convert::{TryFrom, TryInto};
use crate::specs::pos::{Pos, InvalidPos};
use crate::specs::suffix::{SuffixRange, convert_optional_suffix_range};
use crate::lookup::{Indexed, Labelled};
use crate::defs::PosDef;
use crate::error::ItemError;

#[derive(Debug, PartialEq)]
pub struct PosSpec {
    label: String,
    id: Pos,
    suffix_range: Option<SuffixRange>,
    shared: bool,
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
        let suffix_range = convert_optional_suffix_range(def.suffix_range)?;

        Ok(PosSpec {
            label: def.label.to_owned(),
            id: id.to_owned(),
            suffix_range,
            shared: def.shared,
            ordered: def.ordered,
            hidden: def.hidden,
        })
    }
}