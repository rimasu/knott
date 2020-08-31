use crate::specs::GameSpec;
use std::collections::{HashMap, BTreeMap};
use crate::coords::{Pos, QKind, Suffix, QPos, Region};
use crate::state::Shard::{Ordered, Unordered};

use std::fmt;
use std::collections::hash_map::Entry;

trait ShardLike {
    fn len(&self) -> usize;
    fn export_rows(&self, target: &mut Vec<ExportRow>);
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct ExportRow {
    pos: QPos,
    kind: QKind,
    count: u32,
}

impl fmt::Debug for ExportRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:},{:})x{:}", self.pos, self.kind, self.count)
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Key {
    pos_suffix: Suffix,
    kind: QKind,
}

struct UnorderedShard {
    pos: Pos,
    region: Region,
    counts: HashMap<Key, u32>,
}

impl UnorderedShard {
    pub fn new(pos: Pos, region: Region) -> UnorderedShard {
        UnorderedShard {
            pos,
            region,
            counts: HashMap::new(),
        }
    }
}

impl ShardLike for UnorderedShard {
    fn len(&self) -> usize {
        self.counts.len()
    }

    fn export_rows(&self, target: &mut Vec<ExportRow>) {
        for (key, count) in &self.counts {
            let pos = QPos { pos: self.pos, region: self.region, suffix: key.pos_suffix };
            target.push(ExportRow {
                pos,
                kind: key.kind,
                count: *count,
            });
        }
    }
}

struct OrderedShard {
    pos: Pos,
    region: Region,
    counts: BTreeMap<Suffix, QKind>,
}

impl OrderedShard {
    pub fn new(pos: Pos, region: Region) -> OrderedShard {
        OrderedShard {
            pos,
            region,
            counts: BTreeMap::new(),
        }
    }
}

impl ShardLike for OrderedShard {
    fn len(&self) -> usize {
        self.counts.len()
    }

    fn export_rows(&self, target: &mut Vec<ExportRow>) {
        for (pos_suffix, kind) in &self.counts {
            let pos = QPos { pos: self.pos, region: self.region, suffix: *pos_suffix };
            target.push(
                ExportRow {
                    pos,
                    kind: *kind,
                    count: 1,
                }
            );
        }
    }
}

enum MutShard<'a> {
    Ordered(&'a mut OrderedShard),
    Unordered(&'a mut UnorderedShard),
}

struct Regional<T> where T: ShardLike {
    regions: HashMap<Region, T>
}

impl<T> Regional<T> where T: ShardLike {
    pub fn new() -> Regional<T> {
        Regional {
            regions: HashMap::new()
        }
    }
}

impl<T> ShardLike for Regional<T> where T: ShardLike {
    fn len(&self) -> usize {
        self.regions.values().map(ShardLike::len).sum()
    }

    fn export_rows(&self, target: &mut Vec<ExportRow>) {
        for region in self.regions.values() {
            region.export_rows(target)
        }
    }
}

enum Shard {
    Ordered(Regional<OrderedShard>),
    Unordered(Regional<UnorderedShard>),
}

impl ShardLike for Shard {
    fn len(&self) -> usize {
        match self {
            Ordered(s) => s.len(),
            Unordered(s) => s.len(),
        }
    }

    fn export_rows(&self, target: &mut Vec<ExportRow>) {
        match self {
            Ordered(s) => s.export_rows(target),
            Unordered(s) => s.export_rows(target),
        }
    }
}

impl Shard {
    fn find_or_create_shard_mut(&mut self, pos: Pos, region: Region) -> MutShard {
        match self {
            Ordered(shard) => {
                MutShard::Ordered(
                    shard.regions
                        .entry(region)
                        .or_insert_with(|| OrderedShard::new(pos, region))
                )
            }
            Unordered(shard) => {
                MutShard::Unordered(
                    shard.regions
                        .entry(region)
                        .or_insert_with(|| UnorderedShard::new(pos, region))
                )
            }
        }
    }

    fn find_shard_mut(&mut self, region: Region) -> Option<MutShard> {
        match self {
            Ordered(shard) => shard.regions
                .get_mut(&region)
                .map(MutShard::Ordered),

            Unordered(shard) => shard.regions
                .get_mut(&region)
                .map(MutShard::Unordered)
        }
    }
}

pub struct State {
    shards: HashMap<Pos, Shard>
}

impl State {
    pub fn new(spec: &GameSpec) -> State {
        let mut shards = HashMap::new();
        for pos_spec in &spec.pos_specs {
            let shard = if pos_spec.ordered {
                Ordered(Regional::new())
            } else {
                Unordered(Regional::new())
            };
            shards.insert(pos_spec.id, shard);
        }
        State { shards }
    }

    fn len(&self) -> usize {
        self.shards.values().map(ShardLike::len).sum()
    }

    pub fn export_rows(&self) -> Vec<ExportRow> {
        let mut rows = Vec::with_capacity(self.len());
        for shard in self.shards.values() {
            shard.export_rows(&mut rows);
        }
        rows
    }

    pub fn start_tx(&mut self) -> Transaction {
        Transaction {
            state: self
        }
    }
}

pub struct Transaction<'a> {
    state: &'a mut State,
}

pub struct CreatePieces {
    pos: QPos,
    kind: QKind,
    count: u32,
}

pub enum Cmd {
    CreatePieces(CreatePieces)
}

pub enum CmdError {
    NoSuchPos(Pos)
}

impl<'a> Transaction<'a> {
    pub fn apply(&mut self, cmd: &Cmd) -> Result<(), CmdError> {
        match cmd {
            Cmd::CreatePieces(cmd) => self.create_pieces(cmd),
        }
    }

    fn create_pieces(&mut self, cmd: &CreatePieces) -> Result<(), CmdError> {
        match self.find_or_create_shard_mut(cmd.pos.pos, cmd.pos.region)? {
            MutShard::Ordered(ordered) => Self::create_ordered_pieces(cmd, ordered),
            MutShard::Unordered(unordered) => Self::create_unordered_pieces(cmd, unordered),
        }
    }

    fn create_ordered_pieces(cmd: &CreatePieces, ordered: &mut OrderedShard) -> Result<(), CmdError> {
        Ok(())
    }

    fn create_unordered_pieces(cmd: &CreatePieces, shard: &mut UnorderedShard) -> Result<(), CmdError> {
        let key = Key { pos_suffix: cmd.pos.suffix, kind: cmd.kind };
        match shard.counts.entry(key) {
            Entry::Occupied(mut e) => *(e.get_mut()) += cmd.count,
            Entry::Vacant(e) => { e.insert(cmd.count); }
        };
        Ok(())
    }

    fn find_or_create_shard_mut(&mut self, pos: Pos, region: Region) -> Result<MutShard, CmdError> {
        self.find_region_mut(pos)
            .map(|s| s.find_or_create_shard_mut(pos, region))
    }

    fn find_shard_mut(&mut self, pos: Pos, region: Region) -> Result<Option<MutShard>, CmdError> {
        self.find_region_mut(pos)
            .map(|s| s.find_shard_mut(region))
    }

    fn find_region_mut(&mut self, pos: Pos) -> Result<&mut Shard, CmdError> {
        self.state.shards
            .get_mut(&pos)
            .ok_or(CmdError::NoSuchPos(pos))
    }

    pub fn commit(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::defs::{GameDefBuilder, KindDef, SuffixDef, PosDef};
    use crate::specs::GameSpec;
    use std::convert::TryInto;
    use crate::coords::Region;

    struct Fixture {
        spec: GameSpec,
        state: State,
    }

    fn create_fixture() -> Fixture {
        let def = GameDefBuilder::bld("whist")
            .min_players(3)
            .max_players(5)
            .kind(KindDef::bld("card").suffix_range(1, 52))
            .kind(KindDef::bld("leader"))
            .kind(KindDef::bld("to_play"))
            .kind(
                KindDef::bld("suit")
                    .suffix(SuffixDef::bld("hearts"))
                    .suffix(SuffixDef::bld("clubs"))
                    .suffix(SuffixDef::bld("diamonds"))
                    .suffix(SuffixDef::bld("spades")),
            )
            .pos(PosDef::bld("deck").hidden())
            .pos(PosDef::bld("discard").hidden())
            .pos(PosDef::bld("hand").hidden().separate())
            .pos(PosDef::bld("trick").separate())
            .pos(PosDef::bld("trump"))
            .build();

        let spec = def.try_into().unwrap();
        let state = State::new(&spec);

        Fixture {
            spec,
            state,
        }
    }

    impl Fixture {
        fn row<P: AsRef<str>, K: AsRef<str>>(
            &self,
            pos: P,
            pos_suffix: i32,
            region: u16,
            kind: K,
            kind_suffix: i32,
            count: u32,
        ) -> ExportRow {
            let pos = self.spec.pos_specs.find_by_label(pos).unwrap().id;
            let kind = self.spec.kind_specs.find_by_label(kind).unwrap().id;
            let pos = QPos { pos, region: Region(region), suffix: Suffix(pos_suffix) };
            let kind = QKind { kind, suffix: Suffix(kind_suffix) };
            ExportRow { pos, kind, count }
        }

        fn assert_rows(&self, expected_rows: &[ExportRow]) {
            let mut actual_rows = self.state.export_rows();
            actual_rows.sort();
            assert_eq!(
                expected_rows,
                actual_rows.as_slice(),
            )
        }

        fn create_pieces<P: AsRef<str>, K: AsRef<str>>(
            &self,
            pos: P,
            pos_suffix: i32,
            region: u16,
            kind: K,
            kind_suffix: i32,
            count: u32,
        ) -> Cmd {
            let pos = self.spec.pos_specs.find_by_label(pos).unwrap().id;
            let kind = self.spec.kind_specs.find_by_label(kind).unwrap().id;
            let pos = QPos { pos, region: Region(region), suffix: Suffix(pos_suffix) };
            let kind = QKind { kind, suffix: Suffix(kind_suffix) };
            Cmd::CreatePieces(CreatePieces {
                pos,
                kind,
                count,
            })
        }

        fn start_tx(&mut self) -> Transaction {
            self.state.start_tx()
        }

        fn apply(&mut self, cmd: &Cmd) {
            let mut tx = self.state.start_tx();
            tx.apply(&cmd);
            tx.commit();
        }
    }

    #[test]
    fn can_create_pieces_in_unordered_pos() {
        let mut fixture = create_fixture();

        fixture.apply(&fixture.create_pieces("deck", 0, 0, "card", 1, 1));
        fixture.apply(&fixture.create_pieces("deck", 0, 0, "card", 1, 3));

        fixture.assert_rows(
            &vec![
                fixture.row("deck", 0, 0, "card", 1, 4)
            ]
        );
    }
}