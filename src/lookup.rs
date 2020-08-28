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

impl <T> fmt::Debug for LookupTable<T> where T: Debug {
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
        let entries = (0..(max_index+1)).map(|_| None).collect();
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
        if index >= self.entries.len() {
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
    ItemConvert(usize, E)
}

impl<D, S, E> TryFrom<Vec<D>> for LookupTable<S> where D: TryInto<S, Error=E> + Indexed, S: Indexed + Labelled, E: Debug + PartialEq{
    type Error = ConvertError<E>;

    fn try_from(value: Vec<D>) -> Result<Self, Self::Error> {
        let max_index = value.iter()
            .map(|t| t.as_usize())
            .max().unwrap_or(0) as usize;

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