//! Holds a static state store. This static store is required as the PKCS#11
//! interface does not let the libraries to pass around a pointer. Therefore the
//! application cannot keep track of the state, the library need to do it instead.

use std::collections::{HashMap, HashSet};
use std::os::raw::c_ulong;
use std::sync::{Arc, Mutex};

use once_cell::sync::OnceCell;

use super::State;

/// The instance that holds all the states.
static INSTANCE: OnceCell<Mutex<HashMap<c_ulong, Arc<Mutex<State>>>>> = OnceCell::new();

/// Creates a new library state. Returns the index of the newly created state.
/// Returns `None` if the initialization failed, for example
/// if the termux keystore is not reachable.
pub fn new() -> Option<c_ulong> {
    let states = INSTANCE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut states = states.lock().ok()?;
    let keys: HashSet<c_ulong> = states.keys().copied().collect();
    let index = (0..c_ulong::MAX).filter(|i| !&keys.contains(i)).next()?;
    let state = State::from_bridge()?;
    states.insert(index, Arc::new(Mutex::new(state)));
    Some(index)
}

/// Returns the state associated with the given index.
pub fn get(index: c_ulong) -> Option<Arc<Mutex<State>>> {
    let states = INSTANCE.get_or_init(|| Mutex::new(HashMap::new()));
    let states = states.lock().ok()?;
    let state = states.get(&index)?;
    Some(Arc::clone(state))
}

/// Removed the state associated with the index, allowing its resources
/// to be freed.
pub fn remove(index: c_ulong) -> Option<Arc<Mutex<State>>> {
    let states = INSTANCE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut states = states.lock().ok()?;
    states.remove(&index)
}

/// Returns the current number of open states.
pub fn count() -> usize {
    let states = INSTANCE.get_or_init(|| Mutex::new(HashMap::new()));
    let states = states.lock();
    match states {
        Ok(states) => states.len(),
        Err(_) => 0,
    }
}
