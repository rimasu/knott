use std::convert::{TryFrom, TryInto};

use crate::coords::{Kind, Pos};
use crate::defs::GameDef;
use crate::error::Error;
use crate::lookup::LookupTable;
use crate::specs::kind_spec::KindSpec;
use crate::specs::pos_spec::PosSpec;

pub mod kind_spec;
pub mod pos_spec;
pub mod suffix_spec;

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
    input
        .try_into()
        .map_err(|_| Error::InvalidNumPlayers(input))
}

impl TryFrom<GameDef> for GameSpec {
    type Error = Error;

    fn try_from(value: GameDef) -> Result<Self, Self::Error> {
        let mut kind_specs = Vec::with_capacity(value.kind_defs.len());
        for def in value.kind_defs {
            let spec = def.try_into().map_err(Error::InvalidKind)?;
            kind_specs.push(spec);
        }

        let kind_specs: LookupTable<Kind, KindSpec> =
            kind_specs.try_into().map_err(Error::InvalidKindTable)?;

        let mut pos_specs = Vec::with_capacity(value.pos_defs.len());
        for def in value.pos_defs {
            let spec = def.try_into().map_err(Error::InvalidPos)?;
            pos_specs.push(spec);
        }

        let pos_specs: LookupTable<Pos, PosSpec> =
            pos_specs.try_into().map_err(Error::InvalidPosTable)?;

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
    use crate::coords::Suffix;
    use crate::defs::{GameDefBuilder, KindDef, PosDef, SuffixDef};
    use std::convert::TryInto;

    #[test]
    fn can_convert_def_into_spec() {
        let def = GameDefBuilder::new("whist")
            .min_players(3)
            .max_players(5)
            .kind(KindDef::new("card").suffix_range(1, 52))
            .kind(KindDef::new("leader"))
            .kind(KindDef::new("to_play"))
            .kind(
                KindDef::new("suit")
                    .suffix(SuffixDef::new("hearts"))
                    .suffix(SuffixDef::new("clubs"))
                    .suffix(SuffixDef::new("diamonds"))
                    .suffix(SuffixDef::new("spades")),
            )
            .pos(PosDef::new("deck").hidden())
            .pos(PosDef::new("discard").hidden())
            .pos(PosDef::new("hand").hidden().separate())
            .pos(PosDef::new("trick").separate())
            .pos(PosDef::new("trump"))
            .build();

        let spec: GameSpec = def.try_into().unwrap();

        let deck = spec.pos_specs.find_by_label("deck").unwrap();
        assert_eq!(true, deck.hidden);
        assert_eq!(false, deck.ordered);
        assert_eq!(false, deck.separate);

        let suit = spec.kind_specs.find_by_label("suit").unwrap();
        assert_eq!(Some(Suffix(1)), suit.suffixes.find_by_label("hearts"));
        assert_eq!(Some(Suffix(4)), suit.suffixes.find_by_label("spades"));
        assert_eq!(None, suit.suffixes.find_by_label("ravenclaw"));
        assert_eq!(true, suit.suffixes.is_valid(Suffix(1)));
        assert_eq!(true, suit.suffixes.is_valid(Suffix(4)));
        assert_eq!(false, suit.suffixes.is_valid(Suffix(5)));

        let cards = spec.kind_specs.find_by_label("card").unwrap();
        assert_eq!(None, cards.suffixes.find_by_label("ravenclaw"));
        assert_eq!(false, cards.suffixes.is_valid(Suffix(0)));
        assert_eq!(true, cards.suffixes.is_valid(Suffix(1)));
        assert_eq!(true, cards.suffixes.is_valid(Suffix(52)));
        assert_eq!(false, cards.suffixes.is_valid(Suffix(53)));

        let leader = spec.kind_specs.find_by_label("leader").unwrap();
        assert_eq!(false, leader.suffixes.is_valid(Suffix(-1)));
        assert_eq!(true, leader.suffixes.is_valid(Suffix(0)));
        assert_eq!(false, leader.suffixes.is_valid(Suffix(1)));
    }
}
