use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pos(u8);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerNum(u8);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Suffix(i32);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Kind(u8);

fn default_suffix_range() -> Option<SuffixRange> {
    None
}

fn default_false() -> bool {
    false
}

fn ignore_if_false(value: &bool) -> bool {
    value == &false
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SuffixRange {
    min: Suffix,
    max: Suffix,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KindDef {
    label: String,

    id: Kind,

    #[serde(default = "default_suffix_range", skip_serializing_if = "Option::is_none")]
    suffix_range: Option<SuffixRange>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PosDef {
    label: String,
    id: Pos,

    #[serde(default = "default_suffix_range", skip_serializing_if = "Option::is_none")]
    suffix_range: Option<SuffixRange>,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    shared: bool,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    ordered: bool,

    #[serde(default = "default_false", skip_serializing_if = "ignore_if_false")]
    hidden: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameDef {
    label: String,
    min_players: u8,
    max_players: u8,
    kind_defs: Vec<KindDef>,
    pos_defs: Vec<PosDef>,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_round_trip_game_def() {

        let point = GameDef {
            label: "whist".to_owned(),
            min_players: 4,
            max_players: 4,
            kind_defs: vec![
                KindDef {
                    id: Kind(1),
                    label: "card".to_owned(),
                    suffix_range: Some(SuffixRange { min: Suffix(1), max: Suffix(52) }),
                },
                KindDef {
                    id: Kind(2),
                    label: "leader".to_owned(),
                    suffix_range: None,
                },
                KindDef {
                    id: Kind(3),
                    label: "to_play".to_owned(),
                    suffix_range: None,
                },
                KindDef {
                    id: Kind(4),
                    label: "suit".to_owned(),
                    suffix_range: Some(SuffixRange { min: Suffix(1), max: Suffix(4) }),
                }
            ],
            pos_defs: vec![
                PosDef {
                    id: Pos(1),
                    label: "deck".to_owned(),
                    suffix_range: None,
                    shared: true,
                    ordered: false,
                    hidden: true,
                },
                PosDef {
                    id: Pos(2),
                    label: "discard".to_owned(),
                    suffix_range: None,
                    shared: true,
                    ordered: false,
                    hidden: true,
                },
                PosDef {
                    id: Pos(3),
                    label: "hand".to_owned(),
                    suffix_range: None,
                    shared: false,
                    ordered: false,
                    hidden: true,
                },
                PosDef {
                    id: Pos(4),
                    label: "trick".to_owned(),
                    suffix_range: None,
                    shared: false,
                    ordered: false,
                    hidden: false,
                },
                PosDef {
                    id: Pos(5),
                    label: "trump".to_owned(),
                    suffix_range: None,
                    shared: true,
                    ordered: false,
                    hidden: false,
                }
            ],
        };

        let s = serde_yaml::to_string(&point).unwrap();
        assert_eq!(s, "---
label: whist
min_players: 4
max_players: 4
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
        assert_eq!(point, deserialized_point);
    }
}