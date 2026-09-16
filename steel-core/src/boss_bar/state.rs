//! This module contains the base state of a boss bar.
//! This state does not track its viewers.
//! Boss bar implementations can build on top of this state.

use serde::{Deserialize, Serialize};
use steel_utils::boss_bar_properties::{BossBarColor, BossBarFlags, BossBarOverlay};
use text_components::TextComponent;
use uuid::Uuid;

/// Represents the state of a boss bar, without considering its viewers.
/// This stores its name, progress, color, overlay, and flags.
///
/// A boss bar's ID is not persistent: it is not serialized,
/// and upon deserialization, it is assigned a new random UUID.
///
/// Boss bar implementations can build on this state.
#[derive(Debug, Serialize, Deserialize)]
pub struct BossBarState {
    pub(crate) id: Uuid,
    pub(crate) name: TextComponent,
    pub(crate) progress: f32,
    pub(crate) color: BossBarColor,
    pub(crate) overlay: BossBarOverlay,

    #[serde(flatten)]
    pub(crate) flags: BossBarFlags,
}

impl BossBarState {
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
            id: Uuid::new_v4(),
            name,
            progress,
            color,
            overlay,
            flags,
        }
    }

    /// Returns the immutable, non-persistent, unique ID of this boss bar. This is the boss bar's identity, and allows
    /// the client to distinguish between different boss bar instances.
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the displayed name of this boss bar.
    #[must_use]
    pub const fn name(&self) -> &TextComponent {
        &self.name
    }

    /// Sets the displayed name of this boss bar.
    pub fn set_name(&mut self, name: TextComponent) {
        self.name = name;
    }

    /// Returns the progress of this boss bar.
    /// This is a value between `0.0` and `1.0`, where `0.0` means empty and `1.0` means full.
    #[must_use]
    pub const fn progress(&self) -> f32 {
        self.progress
    }

    /// Sets the progress of this boss bar.
    /// This should be a value between `0.0` and `1.0`, where `0.0` means empty and `1.0` means full.
    pub const fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// Returns the color of this boss bar.
    #[must_use]
    pub const fn color(&self) -> BossBarColor {
        self.color
    }

    /// Sets the color of this boss bar.
    pub const fn set_color(&mut self, color: BossBarColor) {
        self.color = color;
    }

    /// Returns the overlay of this boss bar.
    #[must_use]
    pub const fn overlay(&self) -> BossBarOverlay {
        self.overlay
    }

    /// Sets the overlay of this boss bar.
    pub const fn set_overlay(&mut self, overlay: BossBarOverlay) {
        self.overlay = overlay;
    }

    /// Returns the flags of this boss bar.
    #[must_use]
    pub const fn flags(&self) -> BossBarFlags {
        self.flags
    }

    /// Sets the flags of this boss bar.
    pub const fn set_flags(&mut self, flags: BossBarFlags) {
        self.flags = flags;
    }

    /// Adds one or more flags to this boss bar.
    pub fn add_flags(&mut self, flags: BossBarFlags) {
        self.flags |= flags;
    }

    /// Removes one or more flags from this boss bar.
    pub fn remove_flags(&mut self, flags: BossBarFlags) {
        self.flags.remove(flags);
    }
}

impl Clone for BossBarState {
    fn clone(&self) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            progress: self.progress,
            color: self.color,
            overlay: self.overlay,
            flags: self.flags,
        }
    }
}
