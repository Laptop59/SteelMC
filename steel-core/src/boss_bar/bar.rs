//! The main boss bar API. This contains the representation of bossbar that also stores its
//! viewers along with its state.

use crate::boss_bar::impl_boss_bar;
use crate::boss_bar::packets::BossBarOperation;
use crate::boss_bar::state::BossBarState;
use crate::boss_bar::viewers::Viewers;
use crate::player::Player;
use std::sync::Arc;
use steel_utils::boss_bar_properties::{BossBarColor, BossBarFlags, BossBarOverlay};
use text_components::TextComponent;
use uuid::Uuid;

/// Represents an entire boss bar, including its state (name, progress, color,
/// overlay and flags) and its viewers.
pub struct BossBar {
    state: BossBarState,
    viewers: Viewers,
    visible: bool,
}

impl BossBar {
    /// Creates a new boss bar from the specified properties.
    #[must_use]
    pub fn new(
        name: TextComponent,
        progress: f32,
        color: BossBarColor,
        overlay: BossBarOverlay,
        flags: BossBarFlags,
    ) -> Self {
        Self {
            state: BossBarState::new(name, progress, color, overlay, flags),
            viewers: Viewers::new(),
            visible: true,
        }
    }

    impl_boss_bar!();
}

impl BossBar {
    /// Starts showing this boss bar to the given player if visible.
    pub fn add_player(&mut self, player: &Arc<Player>) {
        if self.viewers.add(player) && self.visible {
            player.send_packet(self.state.create_packet(BossBarOperation::Add));
        }
    }

    /// Stops showing this boss bar to the given player if not already invisible.
    pub fn remove_player(&mut self, player: &Arc<Player>) {
        if self.viewers.remove(player) && self.visible {
            player.send_packet(self.state.create_packet(BossBarOperation::Remove));
        }
    }

    /// Stops showing this boss bar to everyone if not already invisible.
    pub fn remove_all_players(&mut self) {
        if self.visible {
            self.update_via_packets(BossBarOperation::Remove);
        }
        self.viewers.clear();
    }

    /// Returns all the online players that are currently being shown this boss bar.
    pub fn viewers(&self) -> impl Iterator<Item = Arc<Player>> {
        self.viewers.iter()
    }
}
