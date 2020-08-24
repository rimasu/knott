use std::convert::{TryFrom, TryInto};
use crate::defs::{SuffixRangeDef, SuffixDef};
use crate::error::{ItemError, SuffixRowError};
use crate::lookup::{LookupTable, Indexed, Labelled};

#[derive(Debug, PartialEq)]
pub struct Suffix(i32);

#[derive(Debug, PartialEq)]
pub struct SuffixRow {
    suffix: Suffix,
    label: String,
}

impl Indexed for SuffixRow {
    fn as_usize(&self) -> usize {
        self.suffix.0 as usize
    }
}

impl Labelled for SuffixRow {
    fn label(&self) -> &str {
        &self.label
    }
}

impl TryFrom<SuffixDef> for SuffixRow {
    type Error = SuffixRowError;
    fn try_from(value: SuffixDef) -> Result<Self, Self::Error> {
        let suffix = Suffix(value.id as i32);
        let label = value.label;
        Ok(SuffixRow { suffix, label })
    }
}

#[derive(Debug, PartialEq)]
pub struct SuffixRange {
    min: Suffix,
    max: Suffix,
}

#[derive(Debug, PartialEq)]
pub enum Suffixes {
    Empty,
    Range(SuffixRange),
    Table(LookupTable<SuffixRow>),
}

impl TryFrom<SuffixRangeDef> for SuffixRange {
    type Error = ItemError;
    fn try_from(value: SuffixRangeDef) -> Result<Self, Self::Error> {
        let min = Suffix(value.min);
        let max = Suffix(value.max);
        if min.0 < max.0 {
            Ok(SuffixRange { min, max })
        } else {
            Err(ItemError::InvalidSuffixRange(value.min, value.max))
        }
    }
}

pub fn convert_suffixes(
    range: Option<SuffixRangeDef>,
    suffixes: Vec<SuffixDef>,
) -> Result<Suffixes, ItemError> {
    if let Some(range) = range {
        if !suffixes.is_empty() {
            Err(ItemError::SuffixesAndRangeDefined)
        } else {
            range.try_into()
                .map(|r| Suffixes::Range(r))
        }
    } else if !suffixes.is_empty() {
        suffixes.try_into()
            .map_err(|e| ItemError::InvalidSuffixRow(e))
            .map(|t| Suffixes::Table(t))
    } else {
        Ok(Suffixes::Empty)
    }
}
