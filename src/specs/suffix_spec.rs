use crate::coords::Suffix;
use crate::defs::{SuffixDef, SuffixRangeDef};
use crate::error::{ItemError, SuffixRowError};
use crate::lookup::{HasId, Labelled, LookupTable};
use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(Clone)]
pub struct SuffixRow {
    pub suffix: Suffix,
    pub label: String,
}

impl HasId<Suffix> for SuffixRow {
    fn id(&self) -> Suffix {
        self.suffix
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

#[derive(Clone)]
pub struct SuffixRange {
    pub min: Suffix,
    pub max: Suffix,
}

impl SuffixRange {
    pub fn contains_suffix(&self, suffix: Suffix) -> bool {
        self.min.0 <= suffix.0 && suffix.0 <= self.max.0
    }
}

impl fmt::Debug for SuffixRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.min, self.max)
    }
}

#[derive(Clone)]
pub enum SuffixSpec {
    Empty,
    Range(SuffixRange),
    Table(LookupTable<Suffix, SuffixRow>),
}

impl SuffixSpec {

    pub fn is_valid(&self, suffix: Suffix) -> bool {
        match self {
            SuffixSpec::Empty => suffix.0 == 0,
            SuffixSpec::Range(range) => range.contains_suffix(suffix),
            SuffixSpec::Table(table) => table.contains_id(&suffix)
        }
    }

    pub fn find_by_label<T: AsRef<str>>(&self, label:T) -> Option<Suffix> {
        match self {
            SuffixSpec::Empty => None,
            SuffixSpec::Range(_) => None,
            SuffixSpec::Table(table) => {
                table.find_by_label(label).map(|r| r.suffix)
            }
        }
    }
}

impl fmt::Debug for SuffixSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuffixSpec::Empty => write!(f, "empty"),
            SuffixSpec::Table(table) => write!(f, "{:?}", table),
            SuffixSpec::Range(range) => write!(f, "{:?}", range),
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
) -> Result<SuffixSpec, ItemError> {
    if let Some(range) = range {
        if !suffixes.is_empty() {
            Err(ItemError::SuffixesAndRangeDefined)
        } else {
            range.try_into().map(SuffixSpec::Range)
        }
    } else if !suffixes.is_empty() {
        let mut rows = Vec::with_capacity(suffixes.len());
        for def in suffixes {
            let row = def.try_into().map_err(ItemError::InvalidSuffixRow)?;

            rows.push(row);
        }
        rows.try_into()
            .map_err(ItemError::InvalidSuffixTable)
            .map(SuffixSpec::Table)
    } else {
        Ok(SuffixSpec::Empty)
    }
}

#[cfg(test)]
mod test {}
