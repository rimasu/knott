use serde::{Deserialize, Serialize};

fn default_suffix_range() -> Option<SuffixRangeDef> {
    None
}

fn default_false() -> bool {
    false
}

fn ignore_if_false(value: &bool) -> bool {
    value == &false
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SuffixRangeDef {
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SuffixDef {
    pub label: String,
    pub id: u32,
}

pub struct SuffixDefBuilder {
    pub label: String,
    pub id: Option<u32>,
}

impl SuffixDef {
    pub fn new<T: AsRef<str>>(label: T) -> SuffixDefBuilder {
        SuffixDefBuilder {
            label: label.as_ref().to_owned(),
            id: None,
        }
    }
}

impl SuffixDefBuilder {
    fn build(self, next_id: u32) -> SuffixDef {
        SuffixDef {
            label: self.label,
            id: self.id.unwrap_or(next_id),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KindDef {
    pub label: String,

    pub id: u32,

    #[serde(
        default = "default_suffix_range",
        skip_serializing_if = "Option::is_none"
    )]
    pub suffix_range: Option<SuffixRangeDef>,

    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub suffixes: Vec<SuffixDef>,
}

pub struct KindDefBuilder {
    label: String,
    id: Option<u32>,
    suffix_range: Option<SuffixRangeDef>,
    suffixes: Vec<SuffixDef>,
}

impl KindDef {
    pub fn new<T: AsRef<str>>(label: T) -> KindDefBuilder {
        KindDefBuilder {
            label: label.as_ref().to_owned(),
            id: None,
            suffix_range: None,
            suffixes: Vec::new(),
        }
    }
}

impl KindDefBuilder {
    pub fn suffix_range(mut self, min: i32, max: i32) -> Self {
        self.suffix_range = Some(SuffixRangeDef { min, max });
        self
    }

    pub fn suffix(mut self, bld: SuffixDefBuilder) -> Self {
        let next_id = (self.suffixes.len() + 1) as u32;
        self.suffixes.push(bld.build(next_id));
        self
    }

    fn build(self, next_id: u32) -> KindDef {
        KindDef {
            label: self.label,
            id: self.id.unwrap_or(next_id),
            suffix_range: self.suffix_range,
            suffixes: self.suffixes,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PosDef {
    pub label: String,
    pub id: u32,

    #[serde(
        default = "default_suffix_range",
        skip_serializing_if = "Option::is_none"
    )]
    pub suffix_range: Option<SuffixRangeDef>,

    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub suffixes: Vec<SuffixDef>,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    pub separate: bool,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    pub ordered: bool,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    pub hidden: bool,
}

pub struct PosDefBuilder {
    label: String,
    id: Option<u32>,
    suffix_range: Option<SuffixRangeDef>,
    suffixes: Vec<SuffixDef>,
    separate: bool,
    ordered: bool,
    hidden: bool,
}

impl PosDef {
    pub fn new<T: AsRef<str>>(label: T) -> PosDefBuilder {
        PosDefBuilder {
            label: label.as_ref().to_owned(),
            id: None,
            suffix_range: None,
            suffixes: Vec::new(),
            separate: false,
            ordered: false,
            hidden: false,
        }
    }
}

impl PosDefBuilder {
    pub fn suffix_range(mut self, min: i32, max: i32) -> Self {
        self.suffix_range = Some(SuffixRangeDef { min, max });
        self
    }

    pub fn suffix(mut self, bld: SuffixDefBuilder) -> Self {
        let next_id = (self.suffixes.len() + 1) as u32;
        self.suffixes.push(bld.build(next_id));
        self
    }

    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub fn ordered(mut self) -> Self {
        self.ordered = true;
        self
    }

    pub fn separate(mut self) -> Self {
        self.separate = true;
        self
    }

    fn build(self, next_id: u32) -> PosDef {
        PosDef {
            label: self.label,
            id: self.id.unwrap_or(next_id),
            suffix_range: self.suffix_range,
            suffixes: self.suffixes,
            separate: self.separate,
            ordered: self.ordered,
            hidden: self.hidden,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameDef {
    pub label: String,
    pub min_players: u32,
    pub max_players: u32,
    pub kind_defs: Vec<KindDef>,
    pub pos_defs: Vec<PosDef>,
}

pub struct GameDefBuilder {
    label: String,
    min_players: u32,
    max_players: u32,
    kind_defs: Vec<KindDef>,
    pos_defs: Vec<PosDef>,
}

impl GameDefBuilder {
    pub fn new<A: AsRef<str>>(label: A) -> GameDefBuilder {
        GameDefBuilder {
            label: label.as_ref().to_owned(),
            min_players: 2,
            max_players: 2,
            kind_defs: Vec::new(),
            pos_defs: Vec::new(),
        }
    }

    pub fn min_players(mut self, num: u32) -> Self {
        self.min_players = num;
        self
    }

    pub fn max_players(mut self, num: u32) -> Self {
        self.max_players = num;
        self
    }

    pub fn kind(mut self, bld: KindDefBuilder) -> Self {
        let next_id = (self.kind_defs.len() + 1) as u32;
        self.kind_defs.push(bld.build(next_id));
        self
    }

    pub fn pos(mut self, bld: PosDefBuilder) -> Self {
        let next_id = (self.pos_defs.len() + 1) as u32;
        self.pos_defs.push(bld.build(next_id));
        self
    }

    pub fn build(self) -> GameDef {
        GameDef {
            label: self.label.to_owned(),
            min_players: self.min_players,
            max_players: self.max_players,
            kind_defs: self.kind_defs,
            pos_defs: self.pos_defs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_build_game_def() {
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

        let s = serde_yaml::to_string(&def).unwrap();
        assert_eq!(
            s,
            "---
label: whist
min_players: 3
max_players: 5
kind_defs:
  - label: card
    id: 1
    suffix_range:
      min: 1
      max: 52
  - label: leader
    id: 2
  - label: to_play
    id: 3
  - label: suit
    id: 4
    suffixes:
      - label: hearts
        id: 1
      - label: clubs
        id: 2
      - label: diamonds
        id: 3
      - label: spades
        id: 4
pos_defs:
  - label: deck
    id: 1
    hidden: true
  - label: discard
    id: 2
    hidden: true
  - label: hand
    id: 3
    separate: true
    hidden: true
  - label: trick
    id: 4
    separate: true
  - label: trump
    id: 5"
        );

        let deserialized_point: GameDef = serde_yaml::from_str(&s).unwrap();
        assert_eq!(def, deserialized_point);
    }
}
