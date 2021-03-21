//! Merkle-Mountain-Range implementation.

use std::marker::PhantomData;

use store::Store;

mod store;
mod utils;

/// Add a dummy hash type for now
pub struct Hash([u8; 32]);

pub struct MerkleMountainRange<'a, T, B>
where
    B: Store<T>,
{
    store: &'a mut B,
    _marker: PhantomData<T>,
}

impl<'a, T, B> MerkleMountainRange<'a, T, B>
where
    B: Store<T>,
{
    pub fn new(store: &'a mut B) -> Self {
        MerkleMountainRange {
            store,
            _marker: PhantomData,
        }
    }
}
