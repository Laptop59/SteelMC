use crate::player::Player;
use rustc_hash::FxHashSet;
use std::sync::{Arc, Weak};

/// A collection acting as a contiguous growable set of weak references to players.
///
/// This is most useful when full iterations occur more frequently than elements are added/removed,
/// as iteration through a contiguous array is fast. Element order is not kept in this set.
///
/// Dead weak references are removed upon cleanup.
#[derive(Debug, Default, Clone)]
pub(super) struct Viewers {
    weaks: Vec<Weak<Player>>,
    addresses: FxHashSet<usize>,
}

impl Viewers {
    /// Creates a new set of viewers.
    pub(super) fn new() -> Self {
        Self {
            weaks: Vec::new(),
            addresses: FxHashSet::default(),
        }
    }

    /// Adds a viewer.
    pub(super) fn add(&mut self, player: &Arc<Player>) -> bool {
        if self.addresses.insert(Arc::as_ptr(player) as usize) {
            self.weaks.push(Arc::downgrade(player));
            true
        } else {
            false
        }
    }

    /// Removes a viewer.
    pub(super) fn remove(&mut self, player: &Arc<Player>) -> bool {
        let addr = Arc::as_ptr(player) as usize;
        if self.addresses.remove(&addr)
            && let Some(i) = self.weaks.iter().position(|v| v.as_ptr() as usize == addr)
        {
            self.weaks.swap_remove(i);
            true
        } else {
            false
        }
    }

    /// Clears all the viewers from the set, leaving it empty.
    pub(super) fn clear(&mut self) {
        self.weaks.clear();
        self.addresses.clear();
    }

    /// Iterates the set without cleaning dead weak references.
    pub(super) fn iter(&self) -> impl Iterator<Item = Arc<Player>> {
        self.weaks.iter().filter_map(Weak::upgrade)
    }

    /// Iterates the set after cleaning dead weak references.
    pub(super) fn iter_cleaned(&mut self) -> impl Iterator<Item = Arc<Player>> {
        self.weaks.retain(|weak| {
            if weak.strong_count() > 0 {
                true
            } else {
                self.addresses.remove(&(weak.as_ptr() as usize));
                false
            }
        });
        self.iter()
    }
}
