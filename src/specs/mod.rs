use std::convert::{TryFrom, TryInto};

use crate::defs::{GameDef, PosDef, SuffixRangeDef, KindDef};
use crate::error::{Error, ItemError};
use crate::specs::kind::{Kind, InvalidKind};
use crate::specs::pos::{Pos, InvalidPos};
use crate::lookup::{Indexed, Labelled, LookupTable};
use crate::specs::suffix::{SuffixRange, convert_optional_suffix_range};
use crate::specs::pos_spec::PosSpec;
use crate::specs::kind_spec::KindSpec;

pub mod suffix;
pub mod pos;
pub mod kind;
pub mod pos_spec;
pub mod kind_spec;

#[derive(Debug, PartialEq)]
pub struct PlayerNum(u8);

pub struct GameSpec {
    label: String,
    min_players: u8,
    max_players: u8,
    kind_specs: LookupTable<KindSpec>,
    pos_specs: LookupTable<PosSpec>,
}

fn convert_player_num(input: u32) -> Result<u8, Error> {
    input.try_into().map_err(|_| Error::InvalidNumPlayers(input))
}

impl TryFrom<GameDef> for GameSpec {
    type Error = Error;

    fn try_from(value: GameDef) -> Result<Self, Self::Error> {
        Ok(GameSpec {
            label: value.label.to_owned(),
            min_players: convert_player_num(value.min_players)?,
            max_players: convert_player_num(value.min_players)?,
            kind_specs: value.kind_defs.try_into().map_err(|e| Error::InvalidKindSpec(e))?,
            pos_specs: value.pos_defs.try_into().map_err(|e| Error::InvalidPosSpec(e))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::defs::{GameDefBuilder, PosFlags};
    use std::convert::TryInto;

    #[test]
    fn can_convert_def_into_spec() {
        let mut bld = GameDefBuilder::new("whist");
        bld.add_range_kind("card", 1, 52)
            .add_kind("leader")
            .add_kind("to_play")
            .add_range_kind("suit", 1, 4)
            .add_pos("deck", PosFlags::HIDDEN | PosFlags::SHARED)
            .add_pos("discard", PosFlags::HIDDEN | PosFlags::SHARED)
            .add_pos("hand", PosFlags::HIDDEN)
            .add_pos("trick", PosFlags::NONE)
            .add_pos("trump", PosFlags::SHARED);
        let def = bld.build();

        let spec: GameSpec = def.try_into().unwrap();
    }
}