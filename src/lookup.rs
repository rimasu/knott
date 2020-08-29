use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::fmt;

pub trait Indexed {
    fn as_usize(&self) -> usize;
}

pub trait Labelled {
    fn label(&self) -> &str;
}

#[derive(PartialEq)]
pub struct LookupTable<V> {
    entries: Vec<Option<V>>,
    label_index: HashMap<String, usize>,
}

impl<T> fmt::Debug for LookupTable<T> where T: Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in &self.entries {
            if let Some(entry) = entry {
                write!(f, "\n{:?}", entry)?;
            }
        }
        Ok(())
    }
}


impl<V> LookupTable<V> where V: Indexed + Labelled {
    pub fn new(max_index: usize) -> LookupTable<V> {
        let entries = (0..(max_index + 1)).map(|_| None).collect();
        LookupTable {
            entries,
            label_index: HashMap::new(),
        }
    }

    pub fn push(&mut self, item: V) {
        let index = item.as_usize();
        self.label_index.insert(item.label().to_owned(), index);
        self.entries[index] = Some(item);
    }

    pub fn find(&self, index: usize) -> Option<&V> {
        if index < self.entries.len() {
            self.entries[index].as_ref()
        } else {
            None
        }
    }

    pub fn find_by_label(&self, label: &str) -> Option<&V> {
        self.label_index
            .get(label)
            .and_then(|i| self.entries[*i].as_ref())
    }
}

#[derive(Debug, PartialEq)]
pub enum ConvertError<E> where E: Debug + PartialEq {
    IndexCollision(usize, usize),
    LabelCollision(usize, String),
    ItemConvert(usize, E),
}

impl<D, S, E> TryFrom<Vec<D>> for LookupTable<S> where D: TryInto<S, Error=E> + Indexed, S: Indexed + Labelled, E: Debug + PartialEq {
    type Error = ConvertError<E>;

    fn try_from(value: Vec<D>) -> Result<Self, Self::Error> {
        let max_index = value
            .iter()
            .map(|t| t.as_usize())
            .max()
            .unwrap_or(0) as usize;

        let mut table = LookupTable::new(max_index);
        let mut idx = 0;
        for def in value {
            idx += 1;
            let spec: S = def.try_into().map_err(|e| ConvertError::ItemConvert(idx, e))?;

            if table.find(spec.as_usize()).is_some() {
                return Err(ConvertError::IndexCollision(idx, spec.as_usize()));
            }

            if table.find_by_label(spec.label()).is_some() {
                return Err(ConvertError::LabelCollision(idx, spec.label().to_owned()));
            }

            table.push(spec);
        }

        Ok(table)
    }
}

#[cfg(test)]
mod test {
    use crate::lookup::{Labelled, Indexed, LookupTable, ConvertError};
    use std::convert::TryInto;
    use crate::lookup::ConvertError::{LabelCollision, IndexCollision};

    #[derive(Debug, PartialEq, Clone)]
    struct Dummy {
        id: usize,
        label: String,
    }

    impl Labelled for Dummy {
        fn label(&self) -> &str {
            &self.label
        }
    }

    impl Indexed for Dummy {
        fn as_usize(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn can_convert_list_to_lookup() {
        let a1 = Dummy { id: 1, label: "A".to_owned() };
        let b4 = Dummy { id: 4, label: "B".to_owned() };
        let c3 = Dummy { id: 3, label: "C".to_owned() };

        let items = vec![a1.clone(), b4.clone(), c3.clone()];

        let lookup: LookupTable<Dummy> = items.try_into().unwrap();

        assert_eq!(Some(&a1), lookup.find_by_label("A"));
        assert_eq!(Some(&b4), lookup.find_by_label("B"));
        assert_eq!(Some(&c3), lookup.find_by_label("C"));
        assert_eq!(None, lookup.find_by_label("D"));

        assert_eq!(None, lookup.find(0));
        assert_eq!(Some(&a1), lookup.find(1));
        assert_eq!(None, lookup.find(2));
        assert_eq!(Some(&c3), lookup.find(3));
        assert_eq!(Some(&b4), lookup.find(4));
        assert_eq!(None, lookup.find(5));
    }

    #[test]
    fn can_not_convert_if_there_is_a_label_collision() {
        let a1 = Dummy { id: 1, label: "A".to_owned() };
        let b4 = Dummy { id: 4, label: "B".to_owned() };
        let b3 = Dummy { id: 3, label: "B".to_owned() };

        let items = vec![a1.clone(), b4.clone(), b3.clone()];

        let result: Result<LookupTable<Dummy>, ConvertError<_>> = items.try_into();

        assert_eq!(
            LabelCollision(3, "B".to_owned()),
            result.unwrap_err()
        )
    }

    #[test]
    fn can_not_convert_if_there_is_a_index_collision() {
        let a1 = Dummy { id: 1, label: "A".to_owned() };
        let b4 = Dummy { id: 4, label: "B".to_owned() };
        let c4 = Dummy { id: 4, label: "C".to_owned() };

        let items = vec![a1.clone(), b4.clone(), c4.clone()];

        let result: Result<LookupTable<Dummy>, ConvertError<_>> = items.try_into();

        assert_eq!(
            IndexCollision(3, 4),
            result.unwrap_err()
        )
    }
}