use std::convert::{TryFrom, TryInto};
use std::fmt;

use crate::coords::{Kind, Pos, InvalidKind, Suffix, InvalidPos};
use crate::defs::{GameDef, KindDef, SuffixDef, PosDef, SuffixRangeDef};
use crate::error::{Error, ItemError, SuffixRowError};
use crate::lookup::{LookupTable, HasId, Labelled};


#[derive(Debug, Clone)]
pub struct KindSpec {
    pub label: String,
    pub id: Kind,
    pub suffixes: SuffixSpec,
}

impl HasId<Kind> for KindSpec {
    fn id(&self) -> Kind {
        self.id
    }
}
impl Labelled for KindSpec {
    fn label(&self) -> &str {
        &self.label
    }
}

impl From<InvalidKind> for ItemError {
    fn from(e: InvalidKind) -> Self {
        ItemError::InvalidId(e.0)
    }
}

impl TryFrom<KindDef> for KindSpec {
    type Error = ItemError;

    fn try_from(def: KindDef) -> Result<Self, Self::Error> {
        let id: Kind = def.id.try_into()?;
        let suffixes = convert_suffixes(def.suffix_range, def.suffixes)?;

        Ok(KindSpec {
            label: def.label,
            id: id.to_owned(),
            suffixes,
        })
    }
}


#[derive(Debug, Clone)]
pub struct PosSpec {
    pub(crate) label: String,
    pub(crate) id: Pos,
    pub(crate) suffixes: SuffixSpec,
    pub(crate) separate: bool,
    pub(crate) ordered: bool,
    pub(crate) hidden: bool,
}

impl HasId<Pos> for PosSpec {
    fn id(&self) -> Pos {
        self.id
    }
}

impl Labelled for PosSpec {
    fn label(&self) -> &str {
        &self.label
    }
}

impl From<InvalidPos> for ItemError {
    fn from(e: InvalidPos) -> Self {
        ItemError::InvalidId(e.0)
    }
}

impl TryFrom<PosDef> for PosSpec {
    type Error = ItemError;

    fn try_from(def: PosDef) -> Result<Self, Self::Error> {
        let id: Pos = def.id.try_into()?;
        let suffixes = convert_suffixes(def.suffix_range, def.suffixes)?;

        Ok(PosSpec {
            label: def.label.to_owned(),
            id: id.to_owned(),
            suffixes,
            separate: def.separate,
            ordered: def.ordered,
            hidden: def.hidden,
        })
    }
}



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
    use crate::lookup::Collision;
    use crate::error::ItemError::InvalidSuffixTable;

    #[test]
    fn can_convert_def_into_spec() {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .kind(KindDef::bld("card").suffix_range(1, 52))
            .kind(KindDef::bld("leader"))
            .kind(KindDef::bld("to_play"))
            .kind(
                KindDef::bld("suit")
                    .suffix(SuffixDef::new("hearts"))
                    .suffix(SuffixDef::new("clubs"))
                    .suffix(SuffixDef::new("diamonds"))
                    .suffix(SuffixDef::new("spades")),
            )
            .pos(PosDef::bld("deck").hidden())
            .pos(PosDef::bld("discard").hidden())
            .pos(PosDef::bld("hand").hidden().separate())
            .pos(PosDef::bld("trick").separate())
            .pos(PosDef::bld("trump"))
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


    #[test]
    fn can_not_convert_game_def_with_colliding_kind_labels() {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .kind(KindDef::bld("card"))
            .kind(KindDef::bld("card"))
            .build();

        let spec: Result<GameSpec, Error> = def.try_into();
        let err = spec.unwrap_err();

        assert_eq!(Error::InvalidKindTable(Collision::LabelCollision("card".to_owned())), err);
    }

    #[test]
    fn can_not_convert_game_def_with_colliding_kind_ids() {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .kind(KindDef::bld("beans").id(2))
            .kind(KindDef::bld("card").id(2))
            .build();

        let spec: Result<GameSpec, Error> = def.try_into();
        let err = spec.unwrap_err();

        assert_eq!(Error::InvalidKindTable(Collision::IdCollision(2.try_into().unwrap())), err);
    }

    #[test]
    fn can_not_convert_game_def_with_colliding_kind_suffix_labels() {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .kind(KindDef::bld("beans")
                .suffix(SuffixDef::new("coke"))
                .suffix(SuffixDef::new("coke"))
            )
            .build();

        let spec: Result<GameSpec, Error> = def.try_into();
        let err = spec.unwrap_err();

        assert_eq!(Error::InvalidKind(
            InvalidSuffixTable(Collision::LabelCollision("coke".to_owned()))
        ), err);
    }

    #[test]
    fn can_not_convert_game_def_with_colliding_pos_labels() {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .pos(PosDef::bld("card"))
            .pos(PosDef::bld("card"))
            .build();

        let spec: Result<GameSpec, Error> = def.try_into();
        let err = spec.unwrap_err();

        assert_eq!(Error::InvalidPosTable(Collision::LabelCollision("card".to_owned())), err);
    }

    #[test]
    fn can_not_convert_game_def_with_colliding_pos_ids() {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .pos(PosDef::bld("beans").id(2))
            .pos(PosDef::bld("card").id(2))
            .build();

        let spec: Result<GameSpec, Error> = def.try_into();
        let err = spec.unwrap_err();

        assert_eq!(Error::InvalidPosTable(Collision::IdCollision(2.try_into().unwrap())), err);
    }


    #[test]
    fn can_not_convert_game_def_with_colliding_pos_suffix_labels() {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .pos(PosDef::bld("beans")
                .suffix(SuffixDef::new("coke"))
                .suffix(SuffixDef::new("coke"))
            )
            .build();

        let spec: Result<GameSpec, Error> = def.try_into();
        let err = spec.unwrap_err();

        assert_eq!(Error::InvalidPos(
            InvalidSuffixTable(Collision::LabelCollision("coke".to_owned()))
        ), err);
    }

}
