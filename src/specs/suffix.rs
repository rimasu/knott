use std::convert::{TryFrom, TryInto};
use crate::defs::SuffixRangeDef;
use crate::error::ItemError;

#[derive(Debug, PartialEq)]
pub struct Suffix(i32);

#[derive(Debug, PartialEq)]
pub struct SuffixRange {
    min: Suffix,
    max: Suffix,
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

pub fn convert_optional_suffix_range(input: Option<SuffixRangeDef>) -> Result<Option<SuffixRange>, ItemError> {
    if let Some(range) = input {
        let range: SuffixRange = range.try_into()?;
        Ok(Some(range))
    } else {
        Ok(None)
    }
}
