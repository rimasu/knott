
use crate::lookup::Collision;
use crate::coords::{Suffix, Pos, Kind};

#[derive(Debug, PartialEq)]
pub enum SuffixRowError {
    Thing
}

#[derive(Debug, PartialEq)]
pub enum ItemError {
    InvalidId(u32),
    SuffixesAndRangeDefined,
    InvalidSuffixRange(i32, i32),
    InvalidSuffixRow(SuffixRowError),
    InvalidSuffixTable(Collision<Suffix>)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNumPlayers(u32),
    InvalidPos(ItemError),
    InvalidPosTable(Collision<Pos>),
    InvalidKind(ItemError),
    InvalidKindTable(Collision<Kind>),
}
