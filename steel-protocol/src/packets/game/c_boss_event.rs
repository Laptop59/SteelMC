use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_BOSS_EVENT;
use steel_utils::boss_bar_properties::{BossBarColor, BossBarFlags, BossBarOverlay};
use text_components::TextComponent;
use uuid::Uuid;

/// Tells the client to add, remove, or update something about a boss bar.
#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_BOSS_EVENT)]
pub struct CBossEvent {
    pub id: Uuid,
    pub operation: BossEventOperation,
}

#[derive(WriteTo, Clone, Debug)]
#[write(as = Dispatched)]
#[repr(i32)]
/// Tells the client what action to perform relating to a boss bar.
pub enum BossEventOperation {
    /// Tells the client to add a new boss bar.
    Add {
        name: TextComponent,
        progress: f32,
        color: BossBarColor,
        overlay: BossBarOverlay,
        flags: BossBarFlags,
    },
    /// Tells the client to remove a boss bar.
    Remove,
    /// Tells the client to update the progress of a boss bar.
    UpdateProgress { progress: f32 },
    /// Tells the client to update the name of a boss bar.
    UpdateName { name: TextComponent },
    /// Tells the client to update the style of a boss bar.
    UpdateStyle {
        color: BossBarColor,
        overlay: BossBarOverlay,
    },
    /// Tells the client to update the properties of a boss bar.
    UpdateProperties { flags: BossBarFlags },
}
