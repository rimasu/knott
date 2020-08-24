use std::convert::{TryFrom, TryInto};

use crate::defs::GameDef;
use crate::error::Error;
use crate::lookup:: LookupTable;
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
    use crate::defs::{GameDefBuilder, PosFlags, SuffixDef, KindDef, PosDef};
    use std::convert::TryInto;

    #[test]
    fn can_convert_def_into_spec() {
        let def = GameDefBuilder::new("whist")
            .min_players(3)
            .max_players(5)
            .kind(KindDef::new("card")
                .suffix_range(1, 52)
            )
            .kind(KindDef::new("leader"))
            .kind(KindDef::new("to_play"))
            .kind(KindDef::new("suit")
                .suffix(SuffixDef::new("hearts"))
                .suffix(SuffixDef::new("clubs"))
                .suffix(SuffixDef::new("diamonds"))
                .suffix(SuffixDef::new("spades"))
            )
            .pos(PosDef::new("deck").hidden())
            .pos(PosDef::new("discard").hidden())
            .pos(PosDef::new("hand").hidden().separate())
            .pos(PosDef::new("trick").separate())
            .pos(PosDef::new("trump"))
            .build();

        let spec: GameSpec = def.try_into().unwrap();
    }
}