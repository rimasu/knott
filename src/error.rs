
use crate::lookup;

#[derive(Debug, PartialEq)]
pub enum ItemError {
    InvalidId(u32),
    InvalidSuffixRange(i32, i32),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNumPlayers(u32),
    InvalidPosSpec(lookup::ConvertError<ItemError>),
    InvalidKindSpec(lookup::ConvertError<ItemError>),
}
