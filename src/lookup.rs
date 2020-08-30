use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HasId<I> {
    fn id(&self) -> I;
}

pub trait Labelled {
    fn label(&self) -> &str;
}

pub struct LookupTable<I, V>
    where
        I: Hash + PartialEq
{
    values: HashMap<I, V>,
    label_index: HashMap<String, I>,
}

impl<I, V> fmt::Debug for LookupTable<I, V>
    where
        I: Hash + PartialEq,
        V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for value in self.values.values() {
            write!(f, "\n{:?}", value)?;
        }
        Ok(())
    }
}

impl<I, V> LookupTable<I, V>
    where
        I: Hash + PartialEq + Eq + Debug,
        V: Labelled + HasId<I>,
{
    pub fn new() -> LookupTable<I, V> {
        LookupTable {
            values: HashMap::new(),
            label_index: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> LookupTable<I, V> {
        LookupTable {
            values: HashMap::with_capacity(capacity),
            label_index: HashMap::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: V) -> Result<(), Collision<I>> {
        if let Entry::Vacant(e) = self.values.entry(item.id()) {
            if let Entry::Vacant(e2) = self.label_index.entry(item.label().to_owned()) {
                e2.insert(item.id());
                e.insert(item);
                Ok(())
            } else {
                Err(Collision::LabelCollision(item.label().to_owned()))
            }
        } else {
            Err(Collision::IdCollision(item.id()))
        }
    }

    pub fn contains_id(&self, index: &I) -> bool {
        self.values.contains_key(index)
    }

    pub fn find(&self, index: &I) -> Option<&V> {
        self.values.get(index)
    }

    pub fn find_by_label<T: AsRef<str>>(&self, label: T) -> Option<&V> {
        self.label_index
            .get(label.as_ref())
            .map(|i| &self.values[i])
    }
}

impl<I, V> TryFrom<Vec<V>> for LookupTable<I, V>
    where
        I: Hash + PartialEq + Eq + Debug,
        V: Labelled + HasId<I>,
{
    type Error = Collision<I>;

    fn try_from(values: Vec<V>) -> Result<Self, Self::Error> {
        let mut table = LookupTable::with_capacity(values.len());
        for value in values {
            table.push(value)?;
        }
        Ok(table)
    }
}

#[derive(Debug, PartialEq)]
pub enum Collision<I>
    where
        I: Debug + PartialEq,
{
    IdCollision(I),
    LabelCollision(String),
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[derive(Debug, Clone, PartialEq)]
    struct Dummy {
        id: u32,
        label: String,
    }

    impl Labelled for Dummy {
        fn label(&self) -> &str {
            &self.label
        }
    }

    impl HasId<u32> for Dummy {
        fn id(&self) -> u32 {
            self.id
        }
    }

    #[test]
    fn can_convert_list_to_lookup() {
        let a1 = Dummy {
            id: 1,
            label: "A".to_owned(),
        };
        let b4 = Dummy {
            id: 4,
            label: "B".to_owned(),
        };
        let c3 = Dummy {
            id: 3,
            label: "C".to_owned(),
        };

        let items = vec![a1.clone(), b4.clone(), c3.clone()];

        let lookup: LookupTable<u32, Dummy> = items.try_into().unwrap();

        assert_eq!(Some(&a1), lookup.find_by_label("A"));
        assert_eq!(Some(&b4), lookup.find_by_label("B"));
        assert_eq!(Some(&c3), lookup.find_by_label("C"));
        assert_eq!(None, lookup.find_by_label("D"));

        assert_eq!(None, lookup.find(&0));
        assert_eq!(Some(&a1), lookup.find(&1));
        assert_eq!(None, lookup.find(&2));
        assert_eq!(Some(&c3), lookup.find(&3));
        assert_eq!(Some(&b4), lookup.find(&4));
        assert_eq!(None, lookup.find(&5));
    }

    #[test]
    fn can_not_convert_if_there_is_a_label_collision() {
        let a1 = Dummy {
            id: 1,
            label: "A".to_owned(),
        };
        let b4 = Dummy {
            id: 4,
            label: "B".to_owned(),
        };
        let b3 = Dummy {
            id: 3,
            label: "B".to_owned(),
        };

        let items = vec![a1.clone(), b4.clone(), b3.clone()];

        let result: Result<LookupTable<u32, Dummy>, Collision<u32>> = items.try_into();

        assert_eq!(
            Collision::LabelCollision("B".to_owned()),
            result.unwrap_err()
        )
    }

    #[test]
    fn can_not_convert_if_there_is_a_index_collision() {
        let a1 = Dummy {
            id: 1,
            label: "A".to_owned(),
        };
        let b4 = Dummy {
            id: 4,
            label: "B".to_owned(),
        };
        let c4 = Dummy {
            id: 4,
            label: "C".to_owned(),
        };

        let items = vec![a1.clone(), b4.clone(), c4.clone()];

        let result: Result<LookupTable<u32, Dummy>, Collision<u32>> = items.try_into();

        assert_eq!(Collision::IdCollision(4), result.unwrap_err())
    }
}
