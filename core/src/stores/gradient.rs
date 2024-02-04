/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Store;
use daggy::petgraph::graph::IndexType;
use daggy::NodeIndex;
use std::any::Any;
use std::collections::BTreeMap;

pub struct GradientStore<Idx = NodeIndex>
where
    Idx: IndexType,
{
    store: BTreeMap<Idx, Box<dyn Any>>,
}

impl<Idx> GradientStore<Idx>
where
    Idx: IndexType,
{
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }

    pub fn or_insert(&mut self, key: Idx, value: Box<dyn Any>) -> &mut dyn Any {
        self.store.entry(key).or_insert(value)
    }
}

impl<Idx, T> Store<Idx, T> for GradientStore<Idx>
where
    Idx: IndexType,
    T: Clone + 'static,
{
    fn get(&self, key: &Idx) -> Option<&T> {
        self.store.get(key).map(|v| v.downcast_ref::<T>().unwrap())
    }

    fn get_mut(&mut self, key: &Idx) -> Option<&mut T> {
        self.store
            .get_mut(key)
            .map(|v| v.downcast_mut::<T>().unwrap())
    }

    fn insert(&mut self, key: Idx, value: T) -> Option<T> {
        self.store
            .insert(key, Box::new(value))
            .map(|v| v.downcast_ref::<T>().unwrap().clone())
    }

    fn remove(&mut self, key: &Idx) -> Option<T> {
        self.store
            .remove(key)
            .map(|v| v.downcast_ref::<T>().unwrap().clone())
    }
}
