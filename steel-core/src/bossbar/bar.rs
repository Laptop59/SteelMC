//! Contains the main boss bar that also stores its viewers along with its state.

use crate::bossbar::packets::BossBarOperation;
use crate::bossbar::state::BossBarState;
use crate::player::Player;
use rustc_hash::FxHashSet;
use std::sync::{Arc, Weak};
use steel_utils::bossbar::{BossBarColor, BossBarFlags, BossBarOverlay};
use text_components::TextComponent;
use uuid::Uuid;

/// Represents an entire boss bar, including its state (name, progress, color,
/// overlay and flags) and its viewers.
pub struct BossBar {
    state: BossBarState,

    viewers: Vec<Weak<Player>>,
    weak_viewer_addresses: FxHashSet<usize>,

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
            viewers: Vec::new(),
            weak_viewer_addresses: FxHashSet::default(),
            visible: true,
        }
    }

    /// Returns the immutable, non-persistent, unique ID of this boss bar. This is the boss bar's identity, and allows
    /// the client to distinguish between different boss bar instances.
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.state.id
    }

    /// Returns the displayed name of this boss bar.
    #[must_use]
    pub const fn name(&self) -> &TextComponent {
        &self.state.name
    }

    /// Sets the displayed name of this boss bar.
    pub fn set_name(&mut self, name: TextComponent) {
        self.state.set_name(name);
        self.update_viewers(BossBarOperation::UpdateName);
    }

    /// Returns the progress of this boss bar.
    /// This is a value between `0.0` and `1.0`, where `0.0` means empty and `1.0` means full.
    #[must_use]
    pub const fn progress(&self) -> f32 {
        self.state.progress
    }

    /// Sets the progress of this boss bar.
    /// This should be a value between `0.0` and `1.0`, where `0.0` means empty and `1.0` means full.
    #[expect(
        clippy::float_cmp,
        reason = "even a small change in the float must be propagated to the viewers"
    )]
    pub fn set_progress(&mut self, progress: f32) {
        let changed = self.state.progress != progress;
        self.state.set_progress(progress);
        if changed {
            self.update_viewers(BossBarOperation::UpdateProgress);
        }
    }

    /// Returns the color of this boss bar.
    #[must_use]
    pub const fn color(&self) -> BossBarColor {
        self.state.color
    }

    /// Sets the color of this boss bar.
    pub fn set_color(&mut self, color: BossBarColor) {
        let changed = self.state.color != color;
        self.state.set_color(color);
        if changed {
            self.update_viewers(BossBarOperation::UpdateStyle);
        }
    }

    /// Returns the overlay of this boss bar.
    #[must_use]
    pub const fn overlay(&self) -> BossBarOverlay {
        self.state.overlay
    }

    /// Sets the overlay of this boss bar.
    pub fn set_overlay(&mut self, overlay: BossBarOverlay) {
        let changed = self.state.overlay != overlay;
        self.state.set_overlay(overlay);
        if changed {
            self.update_viewers(BossBarOperation::UpdateStyle);
        }
    }

    /// Returns the flags of this boss bar.
    #[must_use]
    pub const fn flags(&self) -> BossBarFlags {
        self.state.flags
    }

    /// Sets the flags of this boss bar.
    pub fn set_flags(&mut self, flags: BossBarFlags) {
        let changed = self.state.flags != flags;
        self.state.set_flags(flags);
        if changed {
            self.update_viewers(BossBarOperation::UpdateProperties);
        }
    }

    /// Adds one or more flags to this boss bar.
    pub fn add_flags(&mut self, flags: BossBarFlags) {
        let previous = self.state.flags;
        self.state.add_flags(flags);
        if previous != self.state.flags {
            self.update_viewers(BossBarOperation::UpdateProperties);
        }
    }

    /// Removes one or more flags from this boss bar.
    pub fn remove_flags(&mut self, flags: BossBarFlags) {
        let previous = self.state.flags;
        self.state.remove_flags(flags);
        if previous != self.state.flags {
            self.update_viewers(BossBarOperation::UpdateProperties);
        }
    }
}

impl BossBar {
    /// Returns whether this boss bar is visible.
    #[must_use]
    pub const fn visible(&self) -> bool {
        self.visible
    }

    /// Sets whether this boss bar is visible to its viewers.
    pub fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.update_viewers(if visible {
                BossBarOperation::Add
            } else {
                BossBarOperation::Remove
            });
        }
        self.visible = visible;
    }

    /// Adds a viewer to this boss bar.
    pub fn add_viewer(&mut self, viewer: &Arc<Player>) {
        let addr = Arc::as_ptr(viewer) as usize;
        if self.weak_viewer_addresses.insert(addr) {
            self.viewers.push(Arc::downgrade(viewer));
            if self.visible {
                viewer.send_packet(self.state.packet(BossBarOperation::Add));
            }
        }
    }

    /// Removes a viewer from this boss bar.
    pub fn remove_viewer(&mut self, viewer: &Arc<Player>) {
        let addr = Arc::as_ptr(viewer) as usize;
        if self.weak_viewer_addresses.remove(&addr)
            && let Some(i) = self
                .viewers
                .iter()
                .position(|v| v.as_ptr() as usize == addr)
        {
            self.viewers.swap_remove(i);
        }
        if self.visible {
            viewer.send_packet(self.state.packet(BossBarOperation::Remove));
        }
    }
}

impl BossBar {
    fn update_viewers(&mut self, operation: BossBarOperation) {
        if self.viewers.is_empty() {
            return;
        }

        let packet = self.state.packet(operation);

        let mut i = 0;
        while i < self.viewers.len() {
            if let Some(player) = self.viewers[i].upgrade() {
                player.send_packet(packet.clone());
                i += 1;
            } else {
                let weak = self.viewers.swap_remove(i);
                let addr = weak.as_ptr() as usize;
                self.weak_viewer_addresses.remove(&addr);
            }
        }
    }
}
