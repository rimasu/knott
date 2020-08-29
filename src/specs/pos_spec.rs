use crate::coords::{InvalidPos, Pos};
use crate::defs::PosDef;
use crate::error::ItemError;
use crate::lookup::{HasId, Labelled};
use crate::specs::suffix_spec::{convert_suffixes, SuffixSpec};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct PosSpec {
    pub(crate) label: String,
    pub(crate) id: Pos,
    pub(crate) suffixes: SuffixSpec,
    pub(crate) separate: bool,
    pub(crate) ordered: bool,
    pub(crate) hidden: bool,
}

impl HasId<Pos> for PosSpec {
    fn id(&self) -> Pos {
        self.id
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
