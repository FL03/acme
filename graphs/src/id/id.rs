/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::EntryId;
use crate::NodeIndex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Id<Idx = NodeIndex> {
    id: EntryId,
    index: Idx,
}

impl<Idx> Id<Idx> {
    pub fn new(index: Idx) -> Self {
        Self {
            id: EntryId::new(),
            index,
        }
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn index(&self) -> &Idx {
        &self.index
    }
}

impl<Idx> Default for Id<Idx>
where
    Idx: Default,
{
    fn default() -> Self {
        Self::new(Idx::default())
    }
}

impl<Idx> std::fmt::Display for Id<Idx>
where
    Idx: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}.{}", self.index(), self.id)
        } else {
            write!(f, "{}", self.index())
        }
    }
}