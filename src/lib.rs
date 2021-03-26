//! Merkle-Mountain-Range implementation.

use {std::marker::PhantomData, store::Store};

mod error;
mod store;
mod utils;

/// Merkle-Mountain-Range error codes
pub use error::Error;

pub struct MerkleMountainRange<'a, T, B>
where
    T: Clone,
    B: Store<T>,
{
    store: &'a mut B,
    _marker: PhantomData<T>,
}

impl<'a, T, B> MerkleMountainRange<'a, T, B>
where
    T: Clone,
    B: Store<T>,
{
    pub fn new(store: &'a mut B) -> Self {
        MerkleMountainRange {
            store,
            _marker: PhantomData,
        }
    }
}
