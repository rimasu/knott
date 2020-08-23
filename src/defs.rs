use serde::{Serialize, Deserialize};
use crate::lookup::Indexed;

bitflags! {
    pub struct PosFlags: u8 {
        const NONE  = 0b00000000;
        const HIDDEN  = 0b00000001;
        const SHARED  = 0b00000010;
        const ORDERED = 0b00000100;
    }
}

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

pub struct SuffixTableRowDef {
    pub id: i32,
    pub label: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KindDef {
    pub label: String,

    pub id: u32,

    #[serde(default = "default_suffix_range", skip_serializing_if = "Option::is_none")]
    pub suffix_range: Option<SuffixRangeDef>,
}

impl Indexed for KindDef {
    fn as_usize(&self) -> usize {
        self.id as usize
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PosDef {
    pub label: String,
    pub id: u32,

    #[serde(default = "default_suffix_range", skip_serializing_if = "Option::is_none")]
    pub suffix_range: Option<SuffixRangeDef>,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    pub shared: bool,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    pub ordered: bool,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    pub hidden: bool,
}

impl Indexed for PosDef {
    fn as_usize(&self) -> usize {
        self.id as usize
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

    pub fn min_players(&mut self, num: u32) -> &mut GameDefBuilder {
        self.min_players = num;
        self
    }

    pub fn max_players(&mut self, num: u32) -> &mut GameDefBuilder {
        self.max_players = num;
        self
    }

    pub fn add_kind<T: AsRef<str>>(
        &mut self,
        label: T,
    ) -> &mut GameDefBuilder {
        let id = self.kind_defs.len() + 1;
        let id = id as u32;
        self.kind_defs.push(KindDef {
            label: label.as_ref().to_owned(),
            id,
            suffix_range: None,
        });
        self
    }

    pub fn add_range_kind<T: AsRef<str>>(
        &mut self,
        label: T,
        min: i32,
        max: i32,
    ) -> &mut GameDefBuilder {
        let id = self.kind_defs.len() + 1;
        let id = id as u32;
        self.kind_defs.push(KindDef {
            label: label.as_ref().to_owned(),
            id,
            suffix_range: Some(SuffixRangeDef { min, max }),
        });
        self
    }


    pub fn add_pos<T: AsRef<str>>(
        &mut self,
        label: T,
        flags: PosFlags,
    ) -> &mut GameDefBuilder {
        let id = self.pos_defs.len() + 1;
        let id = id as u32;
        let shared = flags.contains(PosFlags::SHARED);
        let ordered = flags.contains(PosFlags::ORDERED);
        let hidden = flags.contains(PosFlags::HIDDEN);
        self.pos_defs.push(PosDef {
            label: label.as_ref().to_owned(),
            id,
            suffix_range: None,
            shared,
            ordered,
            hidden,
        });
        self
    }

    pub fn add_range_pos<T: AsRef<str>>(
        &mut self,
        label: T,
        min: i32,
        max: i32,
        flags: PosFlags,
    ) -> &mut GameDefBuilder {
        let id = self.pos_defs.len() + 1;
        let id = id as u32;
        let shared = flags.contains(PosFlags::SHARED);
        let ordered = flags.contains(PosFlags::ORDERED);
        let hidden = flags.contains(PosFlags::HIDDEN);
        self.pos_defs.push(PosDef {
            label: label.as_ref().to_owned(),
            id,
            suffix_range: Some(SuffixRangeDef { min, max }),
            shared,
            ordered,
            hidden,
        });
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
        let mut bld = GameDefBuilder::new("whist");

        bld.add_range_kind("card", 1, 52)
            .min_players(3)
            .max_players(5)
            .add_kind("leader")
            .add_kind("to_play")
            .add_range_kind("suit", 1, 4)
            .add_pos("deck", PosFlags::HIDDEN | PosFlags::SHARED)
            .add_pos("discard", PosFlags::HIDDEN | PosFlags::SHARED)
            .add_pos("hand", PosFlags::HIDDEN)
            .add_pos("trick", PosFlags::NONE)
            .add_pos("trump", PosFlags::SHARED);

        let def = bld.build();

        let s = serde_yaml::to_string(&def).unwrap();
        assert_eq!(s, "---
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
    suffix_range:
      min: 1
      max: 4
pos_defs:
  - label: deck
    id: 1
    shared: true
    hidden: true
  - label: discard
    id: 2
    shared: true
    hidden: true
  - label: hand
    id: 3
    hidden: true
  - label: trick
    id: 4
  - label: trump
    id: 5
    shared: true");

        let deserialized_point: GameDef = serde_yaml::from_str(&s).unwrap();
        assert_eq!(def, deserialized_point);
    }
}