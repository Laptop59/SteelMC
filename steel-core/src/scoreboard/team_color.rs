use std::fmt::{Display, Formatter};
use text_components::format::Color;

/// Represents a color that a team can use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TeamColor {
    Black = 0,
    DarkBlue = 1,
    DarkGreen = 2,
    DarkAqua = 3,
    DarkRed = 4,
    DarkPurple = 5,
    Gold = 6,
    Gray = 7,
    DarkGray = 8,
    Blue = 9,
    Green = 10,
    Aqua = 11,
    Red = 12,
    LightPurple = 13,
    Yellow = 14,
    White = 15,
}

impl TeamColor {
    pub const VALUES: [Self; 16] = [
        Self::Black,
        Self::DarkBlue,
        Self::DarkGreen,
        Self::DarkAqua,
        Self::DarkRed,
        Self::DarkPurple,
        Self::Gold,
        Self::Gray,
        Self::DarkGray,
        Self::Blue,
        Self::Green,
        Self::Aqua,
        Self::Red,
        Self::LightPurple,
        Self::Yellow,
        Self::White,
    ];

    /// const Returns the name of this team color.
    pub fn name(self) -> &'static str {
        match self {
            Self::Black => "black",
            Self::DarkBlue => "dark_blue",
            Self::DarkGreen => "dark_green",
            Self::DarkAqua => "dark_aqua",
            Self::DarkRed => "dark_red",
            Self::DarkPurple => "dark_purple",
            Self::Gold => "gold",
            Self::Gray => "gray",
            Self::DarkGray => "dark_gray",
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Aqua => "aqua",
            Self::Red => "red",
            Self::LightPurple => "light_purple",
            Self::Yellow => "yellow",
            Self::White => "white",
        }
    }

    /// Returns the team color identified by the given name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "black" => Some(Self::Black),
            "dark_blue" => Some(Self::DarkBlue),
            "dark_green" => Some(Self::DarkGreen),
            "dark_aqua" => Some(Self::DarkAqua),
            "dark_red" => Some(Self::DarkRed),
            "dark_purple" => Some(Self::DarkPurple),
            "gold" => Some(Self::Gold),
            "gray" => Some(Self::Gray),
            "dark_gray" => Some(Self::DarkGray),
            "blue" => Some(Self::Blue),
            "green" => Some(Self::Green),
            "aqua" => Some(Self::Aqua),
            "red" => Some(Self::Red),
            "light_purple" => Some(Self::LightPurple),
            "yellow" => Some(Self::Yellow),
            "white" => Some(Self::White),
            _ => None,
        }
    }

    /// Returns the color used for formatting text from this team color.
    pub const fn color(self) -> Color {
        match self {
            Self::Black => Color::Black,
            Self::DarkBlue => Color::DarkBlue,
            Self::DarkGreen => Color::DarkGreen,
            Self::DarkAqua => Color::DarkAqua,
            Self::DarkRed => Color::DarkRed,
            Self::DarkPurple => Color::DarkPurple,
            Self::Gold => Color::Gold,
            Self::Gray => Color::Gray,
            Self::DarkGray => Color::DarkGray,
            Self::Blue => Color::Blue,
            Self::Green => Color::Green,
            Self::Aqua => Color::Aqua,
            Self::Red => Color::Red,
            Self::LightPurple => Color::LightPurple,
            Self::Yellow => Color::Yellow,
            Self::White => Color::White,
        }
    }

    /// Returns the team color from its given integral ID.
    pub const fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Black),
            1 => Some(Self::DarkBlue),
            2 => Some(Self::DarkGreen),
            3 => Some(Self::DarkAqua),
            4 => Some(Self::DarkRed),
            5 => Some(Self::DarkPurple),
            6 => Some(Self::Gold),
            7 => Some(Self::Gray),
            8 => Some(Self::DarkGray),
            9 => Some(Self::Blue),
            10 => Some(Self::Green),
            11 => Some(Self::Aqua),
            12 => Some(Self::Red),
            13 => Some(Self::LightPurple),
            14 => Some(Self::Yellow),
            15 => Some(Self::White),
            _ => None,
        }
    }
}

impl Display for TeamColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
