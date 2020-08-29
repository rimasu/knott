use std::convert::{TryFrom, TryInto};

use crate::defs::GameDef;
use crate::error::Error;
use crate::lookup::LookupTable;
use crate::specs::pos_spec::PosSpec;
use crate::specs::kind_spec::KindSpec;
use crate::coords::{Kind, Pos};

pub mod suffix_spec;
pub mod pos_spec;
pub mod kind_spec;

#[derive(Debug, PartialEq)]
pub struct PlayerNum(u8);

#[derive(Debug)]
pub struct GameSpec {
    label: String,
    min_players: u8,
    max_players: u8,
    kind_specs: LookupTable<Kind, KindSpec>,
    pos_specs: LookupTable<Pos, PosSpec>,
}

fn convert_player_num(input: u32) -> Result<u8, Error> {
    input.try_into().map_err(|_| Error::InvalidNumPlayers(input))
}

impl TryFrom<GameDef> for GameSpec {
    type Error = Error;

    fn try_from(value: GameDef) -> Result<Self, Self::Error> {
        let mut kind_specs = Vec::with_capacity(value.kind_defs.len());
        for def in value.kind_defs {
            let spec = def.try_into().map_err(|e| Error::InvalidKind(e))?;
            kind_specs.push(spec);
        }

        let kind_specs: LookupTable<Kind, KindSpec> = kind_specs
            .try_into()
            .map_err(|e| Error::InvalidKindTable(e))?;

        let mut pos_specs = Vec::with_capacity(value.pos_defs.len());
        for def in value.pos_defs {
            let spec = def.try_into().map_err(|e| Error::InvalidPos(e))?;
            pos_specs.push(spec);
        }

        let pos_specs: LookupTable<Pos, PosSpec> = pos_specs
            .try_into()
            .map_err(|e| Error::InvalidPosTable(e))?;

        Ok(GameSpec {
            label: value.label.to_owned(),
            min_players: convert_player_num(value.min_players)?,
            max_players: convert_player_num(value.max_players)?,
            kind_specs,
            pos_specs,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;
    use crate::defs::{GameDefBuilder, SuffixDef, KindDef, PosDef};
    use crate::specs::suffix_spec::{SuffixSpec, SuffixRange, SuffixRow};
    use crate::coords::Suffix;

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


        // let mut kind_specs = LookupTable::new();
        //
        // kind_specs.push(
        //     KindSpec {
        //         label: "card".to_string(),
        //         id: 1.try_into().unwrap(),
        //         suffixes: SuffixSpec::Range(SuffixRange {
        //             min: Suffix(1),
        //             max: Suffix(52)
        //         })
        //     }
        // );
        //
        // kind_specs.push(
        //     KindSpec {
        //         label: "leader".to_string(),
        //         id: 2.try_into().unwrap(),
        //         suffixes: SuffixSpec::Empty,
        //     }
        // );
        //
        // kind_specs.push(
        //     KindSpec {
        //         label: "to_play".to_string(),
        //         id: 3.try_into().unwrap(),
        //         suffixes: SuffixSpec::Empty,
        //     }
        // );
        //
        // let mut suit_table = LookupTable::new();
        // suit_table.push(SuffixRow {
        //     suffix: Suffix(1),
        //     label: "hearts".to_owned()
        // });
        //
        // suit_table.push(SuffixRow {
        //     suffix: Suffix(2),
        //     label: "clubs".to_owned()
        // });
        //
        // suit_table.push(SuffixRow {
        //     suffix: Suffix(3),
        //     label: "diamonds".to_owned()
        // });
        //
        // suit_table.push(SuffixRow {
        //     suffix: Suffix(4),
        //     label: "spades".to_owned()
        // });
        //
        // kind_specs.push(
        //     KindSpec {
        //         label: "suit".to_string(),
        //         id: 4.try_into().unwrap(),
        //         suffixes: SuffixSpec::Table(suit_table),
        //     }
        // );
        //
        // let mut pos_specs = LookupTable::new();
        //
        // pos_specs.push(
        //     PosSpec {
        //         label: "deck".to_string(),
        //         id: 1.try_into().unwrap(),
        //         suffixes: SuffixSpec::Empty,
        //         separate: false,
        //         ordered: false,
        //         hidden: true,
        //     }
        // );
        //
        // pos_specs.push(
        //     PosSpec {
        //         label: "discard".to_string(),
        //         id: 2.try_into().unwrap(),
        //         suffixes: SuffixSpec::Empty,
        //         separate: false,
        //         ordered: false,
        //         hidden: true,
        //     }
        // );
        //
        //
        // pos_specs.push(
        //     PosSpec {
        //         label: "hand".to_string(),
        //         id: 3.try_into().unwrap(),
        //         suffixes: SuffixSpec::Empty,
        //         separate: true,
        //         ordered: false,
        //         hidden: true,
        //     }
        // );
        //
        // pos_specs.push(
        //     PosSpec {
        //         label: "trick".to_string(),
        //         id: 4.try_into().unwrap(),
        //         suffixes: SuffixSpec::Empty,
        //         separate: true,
        //         ordered: false,
        //         hidden: false,
        //     }
        // );
        //
        // pos_specs.push(
        //     PosSpec {
        //         label: "trump".to_string(),
        //         id: 5.try_into().unwrap(),
        //         suffixes: SuffixSpec::Empty,
        //         separate: false,
        //         ordered: false,
        //         hidden: false,
        //     }
        // );
        //
        // assert_eq!(
        //     GameSpec {
        //         label: "whist".to_owned(),
        //         min_players: 3,
        //         max_players: 5,
        //         kind_specs,
        //         pos_specs,
        //     },
        //     spec
        // )
    }
}