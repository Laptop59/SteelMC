//! Provides a way to send boss bar packets to players from a boss bar's state.

use crate::boss_bar::state::BossBarState;
use steel_protocol::packets::game::{BossEventOperation, CBossEvent};

impl BossBarState {
    /// Creates a packet from the boss bar and operation type.
    #[must_use]
    pub fn create_packet(&self, operation: BossBarOperation) -> CBossEvent {
        let operation = match operation {
            BossBarOperation::Add => BossEventOperation::Add {
                name: self.name.clone(),
                progress: self.progress,
                color: self.color,
                overlay: self.overlay,
                flags: self.flags,
            },
            BossBarOperation::Remove => BossEventOperation::Remove,
            BossBarOperation::UpdateProgress => BossEventOperation::UpdateProgress {
                progress: self.progress,
            },
            BossBarOperation::UpdateName => BossEventOperation::UpdateName {
                name: self.name.clone(),
            },
            BossBarOperation::UpdateStyle => BossEventOperation::UpdateStyle {
                color: self.color,
                overlay: self.overlay,
            },
            BossBarOperation::UpdateProperties => {
                BossEventOperation::UpdateProperties { flags: self.flags }
            }
        };
        CBossEvent {
            id: self.id,
            operation,
        }
    }
}

/// Represents the operation the boss bar packet should do to the client.
pub enum BossBarOperation {
    /// Adds this boss bar as a new one to the client.
    /// This uses the boss bar's current state.
    Add,

    /// Removes this boss bar from the client.
    Remove,

    /// Updates the progress of this boss bar for the client.
    UpdateProgress,

    /// Updates the name of this boss bar for the client.
    UpdateName,

    /// Updates the style of this boss bar for the client.
    UpdateStyle,

    /// Updates the properties of this boss bar for the client.
    UpdateProperties,
}
