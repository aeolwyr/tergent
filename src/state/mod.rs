//! Provides functionality to store the library state.

use crate::bridge;
use crate::key::{self, Key};

mod store;
pub use store::*;

/// Represents the state of the library. Each session should
/// have a single state associated with it.
pub struct State {
    keys: Vec<Key>,
    search_index: usize,
    search_for_keys: bool,
    search_id: Option<String>,
    sign_index: usize,
}

impl State {
    /// Initializes the library state by fetching the keys from termux keystore.
    /// Returns `None` if this fetch has failed.
    fn from_bridge() -> Option<Self> {
        let json = bridge::list_keys().ok()?;
        let keys = key::json_to_list(json)?;
        Some(State {
            keys,
            search_index: 0,
            search_for_keys: false,
            search_id: None,
            sign_index: 0,
        })
    }

    /// Initializes the search operation. `search_for_keys` indicates if the search
    /// is for public/private keys, false will mean it is searching for something else,
    /// such as certificates.
    /// The optional field `id` limits the search to the given label.
    pub fn find_init(&mut self, search_for_keys: bool, id: Option<String>) {
        self.search_index = 0;
        self.search_for_keys = search_for_keys;
        self.search_id = id;
    }

    /// Continues a previously initiated search, returning the next key. Returns `None`
    /// if there are no more results left.
    pub fn find_next(&mut self) -> Option<usize> {
        let index = self
            .keys
            .iter()
            .enumerate()
            .filter(|&_| self.search_for_keys)
            .filter(|&k| match &self.search_id {
                Some(id) => k.1.label() == id,
                None => true,
            })
            .skip(self.search_index)
            .next()
            .map(|(i, &_)| i);
        if let Some(i) = index {
            self.search_index = i + 1;
        }
        index
    }

    /// Fetches a key by its index.
    pub fn get_key(&self, index: usize) -> Option<&Key> {
        self.keys.get(index)
    }

    /// Sets up the store so that it can be used to sign with the provided key later.
    pub fn sign_init(&mut self, index: usize) {
        self.sign_index = index;
    }

    /// Fetches the signing key which was previously set by `sign_init`.
    pub fn get_sign_key(&self) -> Option<&Key> {
        self.keys.get(self.sign_index)
    }
}
