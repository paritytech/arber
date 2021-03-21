//! Merkle-Mountain-Range storage
use crate::Hash;

pub trait Store<T> {
    fn append(&mut self, data: &T, hashes: &[Hash]);
}

pub(crate) struct VecStore<T> {
    pub data: Vec<T>,
    pub hashes: Vec<Hash>,
}

impl<T> Store<T> for VecStore<T> {
    fn append(&mut self, data: &T, hashes: &[Hash]) {
        todo!()
    }
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        VecStore {
            data: vec![],
            hashes: vec![],
        }
    }
}
