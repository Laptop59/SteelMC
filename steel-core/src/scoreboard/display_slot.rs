use crate::scoreboard::team_color::TeamColor;
use std::fmt::{Display, Formatter};

/// Represents a way to display an objective.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DisplaySlot {
    List,
    Sidebar,
    BelowName,
    Team(TeamColor),
}

impl DisplaySlot {
    /// Returns the name used by this display slot.
    pub const fn name(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Sidebar => "sidebar",
            Self::BelowName => "below_name",
            Self::Team(TeamColor::Black) => "sidebar.team.black",
            Self::Team(TeamColor::DarkBlue) => "sidebar.team.dark_blue",
            Self::Team(TeamColor::DarkGreen) => "sidebar.team.dark_green",
            Self::Team(TeamColor::DarkAqua) => "sidebar.team.dark_aqua",
            Self::Team(TeamColor::DarkRed) => "sidebar.team.dark_red",
            Self::Team(TeamColor::DarkPurple) => "sidebar.team.dark_purple",
            Self::Team(TeamColor::Gold) => "sidebar.team.gold",
            Self::Team(TeamColor::Gray) => "sidebar.team.gray",
            Self::Team(TeamColor::DarkGray) => "sidebar.team.dark_gray",
            Self::Team(TeamColor::Blue) => "sidebar.team.blue",
            Self::Team(TeamColor::Green) => "sidebar.team.green",
            Self::Team(TeamColor::Aqua) => "sidebar.team.aqua",
            Self::Team(TeamColor::Red) => "sidebar.team.red",
            Self::Team(TeamColor::LightPurple) => "sidebar.team.light_purple",
            Self::Team(TeamColor::Yellow) => "sidebar.team.yellow",
            Self::Team(TeamColor::White) => "sidebar.team.white",
        }
    }

    /// Returns the display slot identified by the given name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "list" => Some(Self::List),
            "sidebar" => Some(Self::Sidebar),
            "below_name" => Some(Self::BelowName),
            "sidebar.team.black" => Some(Self::Team(TeamColor::Black)),
            "sidebar.team.dark_blue" => Some(Self::Team(TeamColor::DarkBlue)),
            "sidebar.team.dark_green" => Some(Self::Team(TeamColor::DarkGreen)),
            "sidebar.team.dark_aqua" => Some(Self::Team(TeamColor::DarkAqua)),
            "sidebar.team.dark_red" => Some(Self::Team(TeamColor::DarkRed)),
            "sidebar.team.dark_purple" => Some(Self::Team(TeamColor::DarkPurple)),
            "sidebar.team.gold" => Some(Self::Team(TeamColor::Gold)),
            "sidebar.team.gray" => Some(Self::Team(TeamColor::Gray)),
            "sidebar.team.dark_gray" => Some(Self::Team(TeamColor::DarkGray)),
            "sidebar.team.blue" => Some(Self::Team(TeamColor::Blue)),
            "sidebar.team.green" => Some(Self::Team(TeamColor::Green)),
            "sidebar.team.aqua" => Some(Self::Team(TeamColor::Aqua)),
            "sidebar.team.red" => Some(Self::Team(TeamColor::Red)),
            "sidebar.team.light_purple" => Some(Self::Team(TeamColor::LightPurple)),
            "sidebar.team.yellow" => Some(Self::Team(TeamColor::Yellow)),
            "sidebar.team.white" => Some(Self::Team(TeamColor::White)),
            _ => None,
        }
    }

    /// Returns the integral ID of this display slot.
    pub const fn id(self) -> i32 {
        match self {
            Self::List => 0,
            Self::Sidebar => 1,
            Self::BelowName => 2,
            Self::Team(color) => 3 + color as i32,
        }
    }

    /// Returns the integral ID of this display slot.
    pub fn from_id(id: i32) -> Option<DisplaySlot> {
        match id {
            0 => Some(Self::List),
            1 => Some(Self::Sidebar),
            2 => Some(Self::BelowName),
            3.. => Some(Self::Team(TeamColor::from_id(id - 3)?)),
            _ => None,
        }
    }
}

impl Display for DisplaySlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
