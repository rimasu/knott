use std::convert::{TryFrom, TryInto};
use crate::defs::{SuffixRangeDef, SuffixDef};
use crate::error::{ItemError, SuffixRowError};
use crate::lookup::{LookupTable, Indexed, Labelled};
use std::fmt;

#[derive(PartialEq)]
pub struct Suffix(pub i32);

impl fmt::Debug for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:+}", self.0)
    }
}

#[derive(PartialEq)]
pub struct SuffixRow {
    pub suffix: Suffix,
    pub label: String,
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

impl fmt::Debug for SuffixRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t{:?} {:?}", self.suffix, self.label)
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

#[derive(PartialEq)]
pub struct SuffixRange {
    pub min: Suffix,
    pub max: Suffix,
}

impl fmt::Debug for SuffixRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.min, self.max)
    }
}

#[derive(PartialEq)]
pub enum Suffixes {
    Empty,
    Range(SuffixRange),
    Table(LookupTable<SuffixRow>),
}

impl fmt::Debug for Suffixes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suffixes::Empty => write!(f, "empty"),
            Suffixes::Table(table) => write!(f, "{:?}", table),
            Suffixes::Range(range)=> write!(f, "{:?}", range)
        }
    }
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
