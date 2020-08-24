
use crate::lookup;

#[derive(Debug, PartialEq)]
pub enum SuffixRowError {
}

#[derive(Debug, PartialEq)]
pub enum ItemError {
    InvalidId(u32),
    SuffixesAndRangeDefined,
    InvalidSuffixRange(i32, i32),
    InvalidSuffixRow(lookup::ConvertError<SuffixRowError>)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNumPlayers(u32),
    InvalidPosSpec(lookup::ConvertError<ItemError>),
    InvalidKindSpec(lookup::ConvertError<ItemError>),
}
