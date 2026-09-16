//! This module contains the boss bar manager that will be used by the `/bossbar` command.

use crate::boss_bar::custom::CustomBossBar;
use rustc_hash::FxHashMap;
use steel_utils::Identifier;
use steel_utils::locks::SyncRwLock;

/// A manager for the `/bossbar` command to add, remove, and modify custom boss bars.
pub struct BossBarManager {
    boss_bars: SyncRwLock<FxHashMap<Identifier, CustomBossBar>>,
}
