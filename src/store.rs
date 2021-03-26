//! Merkle-Mountain-Range storage

use {crate::Error, primitive_types::H256};

pub trait Store<T>
where
    T: Clone,
{
    fn append(&mut self, data: &T, hashes: &[H256]) -> Result<(), Error>;
}

#[derive(Clone, Debug)]
pub(crate) struct VecStore<T> {
    pub data: Vec<T>,
    pub hashes: Vec<H256>,
}

impl<T> Store<T> for VecStore<T>
where
    T: Clone,
{
    fn append(&mut self, data: &T, hashes: &[H256]) -> Result<(), Error> {
        self.data.push(data.clone());
        self.hashes.extend_from_slice(hashes);

        Ok(())
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
