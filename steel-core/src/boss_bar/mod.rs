//! Module for boss bars and their manager.

pub mod bar;
pub mod custom;
pub mod manager;
pub mod packets;
pub mod state;

mod viewers;

macro_rules! impl_boss_bar {
    () => {
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
            self.update_via_packets(BossBarOperation::UpdateName);
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
            reason = "even a small change in the progress must be propagated to the viewers"
        )]
        pub fn set_progress(&mut self, progress: f32) {
            let changed = self.state.progress != progress;
            self.state.set_progress(progress);
            if changed {
                self.update_via_packets(BossBarOperation::UpdateProgress);
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
                self.update_via_packets(BossBarOperation::UpdateStyle);
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
                self.update_via_packets(BossBarOperation::UpdateStyle);
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
                self.update_via_packets(BossBarOperation::UpdateProperties);
            }
        }

        /// Adds one or more flags to this boss bar.
        pub fn add_flags(&mut self, flags: BossBarFlags) {
            let previous = self.state.flags;
            self.state.add_flags(flags);
            if previous != self.state.flags {
                self.update_via_packets(BossBarOperation::UpdateProperties);
            }
        }

        /// Returns whether this boss bar is visible.
        #[must_use]
        pub const fn visible(&self) -> bool {
            self.visible
        }

        /// Sets whether this boss bar is visible to its viewers.
        pub fn set_visible(&mut self, visible: bool) {
            if self.visible != visible {
                self.update_via_packets(if visible {
                    BossBarOperation::Add
                } else {
                    BossBarOperation::Remove
                });
            }
            self.visible = visible;
        }

        fn update_via_packets(&mut self, operation: BossBarOperation) {
            let packet = self.state.create_packet(operation);
            for player in self.viewers.iter_cleaned() {
                player.send_packet(packet.clone());
            }
        }
    };
}

use impl_boss_bar;
