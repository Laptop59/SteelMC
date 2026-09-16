//! This module contains the boss bar manager that will be used by the `/bossbar` command.

use crate::boss_bar::impl_boss_bar;
use crate::boss_bar::packets::BossBarOperation;
use crate::boss_bar::state::BossBarState;
use crate::boss_bar::viewers::Viewers;
use crate::entity::Entity;
use crate::player::Player;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use steel_utils::Identifier;
use steel_utils::boss_bar_properties::{BossBarColor, BossBarFlags, BossBarOverlay};
use steel_utils::locks::SyncRwLock;
use text_components::TextComponent;
use uuid::Uuid;

/// A manager for the `/bossbar` command to add, remove, and modify custom boss bars.
pub struct BossBarManager {
    boss_bars: SyncRwLock<FxHashMap<Identifier, CustomBossBar>>,
}

/// A custom boss bar that is controlled by `/bossbar`.
#[derive(Serialize, Deserialize)]
pub struct CustomBossBar {
    #[serde(flatten)]
    state: BossBarState,

    visible: bool,
    value: i32,
    max: i32,

    #[serde(skip)]
    viewers: Viewers,

    players: FxHashSet<Uuid>,
}

impl CustomBossBar {
    /// Creates a new boss bar from the specified properties.
    #[must_use]
    pub fn new(
        name: TextComponent,
        value: i32,
        max: i32,
        color: BossBarColor,
        overlay: BossBarOverlay,
        flags: BossBarFlags,
        players: FxHashSet<Uuid>,
    ) -> Self {
        Self {
            state: BossBarState::new(
                name,
                Self::calculate_progress(value, max),
                color,
                overlay,
                flags,
            ),
            viewers: Viewers::new(),
            players,
            value,
            max,
            visible: true,
        }
    }

    impl_boss_bar!();

    /// Fetches the current value of the bossbar.
    pub const fn value(&mut self) -> i32 {
        self.value
    }

    /// Sets the value of the bossbar. Progress is then recalculated relative to the current maximum value.
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
        self.set_progress(Self::calculate_progress(value, self.max));
    }

    /// Fetches the current value of the bossbar.
    pub const fn max(&mut self) -> i32 {
        self.max
    }

    /// Sets the maximum value of the bossbar. Progress is then recalculated relative to the current value.
    pub fn set_max(&mut self, max: i32) {
        self.max = max;
        self.set_progress(Self::calculate_progress(self.value, max));
    }

    const fn calculate_progress(value: i32, max: i32) -> f32 {
        (value as f32 / max as f32).clamp(0.0, 1.0)
    }
}

impl CustomBossBar {
    /// Starts showing this boss bar to the given player if visible.
    pub fn add_player(&mut self, player: &Arc<Player>) {
        if self.viewers.add(player) && self.visible {
            player.send_packet(self.state.create_packet(BossBarOperation::Add));
        }
        self.players.insert(player.uuid());
    }

    /// Stops showing this boss bar to the given player if not already invisible.
    pub fn remove_player(&mut self, player: &Arc<Player>) {
        if self.viewers.remove(player) && self.visible {
            player.send_packet(self.state.create_packet(BossBarOperation::Remove));
        }
        self.players.remove(&player.uuid());
    }

    /// Stops showing this boss bar to everyone if not already invisible.
    pub fn remove_all_players(&mut self) {
        if self.visible {
            self.update_via_packets(BossBarOperation::Remove);
        }
        self.viewers.clear();
        self.players.clear();
    }

    /// Returns all the online players that are currently being shown this boss bar.
    pub fn viewers(&self) -> impl Iterator<Item = Arc<Player>> {
        self.viewers.iter()
    }

    /// Returns all the players that are able to see this boss bar when online.
    pub fn players(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.players.iter().copied()
    }
}
