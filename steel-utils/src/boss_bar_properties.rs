//! This module contains the types for various boss bar properties.

use crate::codec::VarInt;
use crate::serial::WriteTo;
use bitflags::bitflags;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::io::Write;
use std::{fmt, io};
use text_components::format::Color;

/// Represents the color of a boss bar.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BossBarColor {
    /// Represents the pink boss bar color.
    Pink = 0,

    /// Represents the blue boss bar color.
    Blue = 1,

    /// Represents the red boss bar color.
    Red = 2,

    /// Represents the green boss bar color.
    Green = 3,

    /// Represents the yellow boss bar color.
    Yellow = 4,

    /// Represents the purple boss bar color.
    Purple = 5,

    /// Represents the white boss bar color.
    White = 6,
}

impl BossBarColor {
    /// All the possible colors of a boss bar.
    pub const VALUES: [Self; 7] = [
        Self::Pink,
        Self::Blue,
        Self::Red,
        Self::Green,
        Self::Yellow,
        Self::Purple,
        Self::White,
    ];

    /// Gets the name of this color.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Pink => "pink",
            Self::Blue => "blue",
            Self::Red => "red",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Purple => "purple",
            Self::White => "white",
        }
    }

    /// Gets the formatting color of this color.
    #[must_use]
    pub const fn color(self) -> Color {
        match self {
            Self::Pink => Color::Red,
            Self::Blue => Color::Blue,
            Self::Red => Color::DarkRed,
            Self::Green => Color::Green,
            Self::Yellow => Color::Yellow,
            Self::Purple => Color::DarkBlue,
            Self::White => Color::White,
        }
    }
}

impl WriteTo for BossBarColor {
    fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        VarInt(*self as i32).write(writer)
    }
}

/// Represents the overlay of a boss bar.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BossBarOverlay {
    /// Represents an overlay without any notches.
    Progress = 0,

    /// Represents an overlay with 6 notches.
    #[serde(rename = "notched_6")]
    Notched6 = 1,

    /// Represents an overlay with 10 notches.
    #[serde(rename = "notched_10")]
    Notched10 = 2,

    /// Represents an overlay with 12 notches.
    #[serde(rename = "notched_12")]
    Notched12 = 3,

    /// Represents an overlay with 20 notches.
    #[serde(rename = "notched_20")]
    Notched20 = 4,
}

impl BossBarOverlay {
    /// All the possible overlays of a boss bar.
    pub const VALUES: [Self; 5] = [
        Self::Progress,
        Self::Notched6,
        Self::Notched10,
        Self::Notched12,
        Self::Notched20,
    ];

    /// Gets the name of this overlay.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Notched6 => "notched_6",
            Self::Notched10 => "notched_10",
            Self::Notched12 => "notched_12",
            Self::Notched20 => "notched_20",
        }
    }
}

impl WriteTo for BossBarOverlay {
    fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        VarInt(*self as i32).write(writer)
    }
}

bitflags! {
    /// Represents the flags of a boss bar.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BossBarFlags: u8 {
        /// Whether this boss bar should darken the player's screen, when visible.
        #[bitflags(flag_name = "darken_screen")]
        const DARKEN_SCREEN = 1 << 0;
        /// Whether this boss bar should play boss music, when visible.
        #[bitflags(flag_name = "play_boss_music")]
        const PLAY_BOSS_MUSIC = 1 << 1;
        /// Whether this boss bar should create world fog, when visible.
        #[bitflags(flag_name = "create_world_fog")]
        const CREATE_WORLD_FOG = 1 << 2;
    }
}

impl Serialize for BossBarFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.bits().count_ones() as usize))?;
        for (name, _) in self.iter_names() {
            map.serialize_entry(name, &true)?;
        }
        map.end()
    }
}

struct BossBarFlagsVisitor;

impl<'de> Deserialize<'de> for BossBarFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(BossBarFlagsVisitor)
    }
}

impl<'de> Visitor<'de> for BossBarFlagsVisitor {
    type Value = BossBarFlags;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a map containing boss bar flag names and whether they are set")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut flags = BossBarFlags::empty();
        while let Some((flag_name, value)) = map.next_entry::<&str, bool>()? {
            let flag = BossBarFlags::from_name(flag_name)
                .ok_or_else(|| Error::custom(format!("invalid boss bar flag: {flag_name}")))?;
            if value {
                flags |= flag;
            }
        }
        Ok(flags)
    }
}

impl WriteTo for BossBarFlags {
    fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        self.bits().write(writer)
    }
}

impl Default for BossBarFlags {
    fn default() -> Self {
        Self::empty()
    }
}
