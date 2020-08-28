use std::convert::{TryFrom, TryInto};

use crate::defs::GameDef;
use crate::error::Error;
use crate::lookup::LookupTable;
use crate::specs::pos_spec::PosSpec;
use crate::specs::kind_spec::KindSpec;

pub mod suffix;
pub mod pos;
pub mod kind;
pub mod pos_spec;
pub mod kind_spec;

#[derive(Debug, PartialEq)]
pub struct PlayerNum(u8);

#[derive(Debug, PartialEq)]
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
            max_players: convert_player_num(value.max_players)?,
            kind_specs: value.kind_defs.try_into().map_err(|e| Error::InvalidKindSpec(e))?,
            pos_specs: value.pos_defs.try_into().map_err(|e| Error::InvalidPosSpec(e))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::defs::{GameDefBuilder, SuffixDef, KindDef, PosDef};
    use std::convert::TryInto;
    use crate::specs::suffix::{Suffixes, SuffixRange, Suffix, SuffixRow};

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


        let mut kind_specs = LookupTable::new(4);

        kind_specs.push(
            KindSpec {
                label: "card".to_string(),
                id: 1.try_into().unwrap(),
                suffixes: Suffixes::Range(SuffixRange {
                    min: Suffix(1),
                    max: Suffix(52)
                })
            }
        );

        kind_specs.push(
            KindSpec {
                label: "leader".to_string(),
                id: 2.try_into().unwrap(),
                suffixes: Suffixes::Empty,
            }
        );

        kind_specs.push(
            KindSpec {
                label: "to_play".to_string(),
                id: 3.try_into().unwrap(),
                suffixes: Suffixes::Empty,
            }
        );

        let mut suit_table = LookupTable::new(4);
        suit_table.push(SuffixRow {
            suffix: Suffix(1),
            label: "hearts".to_owned()
        });

        suit_table.push(SuffixRow {
            suffix: Suffix(2),
            label: "clubs".to_owned()
        });

        suit_table.push(SuffixRow {
            suffix: Suffix(3),
            label: "diamonds".to_owned()
        });

        suit_table.push(SuffixRow {
            suffix: Suffix(4),
            label: "spades".to_owned()
        });

        kind_specs.push(
            KindSpec {
                label: "suit".to_string(),
                id: 4.try_into().unwrap(),
                suffixes: Suffixes::Table(suit_table),
            }
        );

        let mut pos_specs = LookupTable::new(5);

        pos_specs.push(
            PosSpec {
                label: "deck".to_string(),
                id: 1.try_into().unwrap(),
                suffixes: Suffixes::Empty,
                separate: false,
                ordered: false,
                hidden: true,
            }
        );

        pos_specs.push(
            PosSpec {
                label: "discard".to_string(),
                id: 2.try_into().unwrap(),
                suffixes: Suffixes::Empty,
                separate: false,
                ordered: false,
                hidden: true,
            }
        );


        pos_specs.push(
            PosSpec {
                label: "hand".to_string(),
                id: 3.try_into().unwrap(),
                suffixes: Suffixes::Empty,
                separate: true,
                ordered: false,
                hidden: true,
            }
        );

        pos_specs.push(
            PosSpec {
                label: "trick".to_string(),
                id: 4.try_into().unwrap(),
                suffixes: Suffixes::Empty,
                separate: true,
                ordered: false,
                hidden: false,
            }
        );

        pos_specs.push(
            PosSpec {
                label: "trump".to_string(),
                id: 5.try_into().unwrap(),
                suffixes: Suffixes::Empty,
                separate: false,
                ordered: false,
                hidden: false,
            }
        );

        assert_eq!(
            GameSpec {
                label: "whist".to_owned(),
                min_players: 3,
                max_players: 5,
                kind_specs,
                pos_specs,
            },
            spec
        )
    }
}