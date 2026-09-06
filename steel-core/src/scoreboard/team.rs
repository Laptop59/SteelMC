use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use text_components::format::Format;
use text_components::TextComponent;
use crate::scoreboard::team_color::TeamColor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub players: FxHashSet<Box<str>>,
    pub display_name: TextComponent,
    pub player_prefix: TextComponent,
    pub player_suffix: TextComponent,
    pub allow_friendly_fire: bool,
    pub see_friendly_invisibles: bool,
    pub name_tag_visibility: TeamVisibility,
    pub death_message_visibility: TeamVisibility,
    pub color: Option<TeamColor>,
    pub collision_rule: TeamCollisionRule,
    pub display_name_style: Format
}

impl Team {
    pub fn new(name: &str) -> Self {
        let name_text_component = TextComponent::plain(name);
        Self {
            players: FxHashSet::default(),
            display_name: name_text_component.clone(),
            player_prefix: TextComponent::new(),
            player_suffix: TextComponent::new(),
            allow_friendly_fire: true,
            see_friendly_invisibles: true,
            name_tag_visibility: TeamVisibility::Always,
            death_message_visibility: TeamVisibility::Always,
            color: None,
            collision_rule: TeamCollisionRule::Always,
            display_name_style: todo!()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeamVisibility {
    Always = 0,
    Never = 1,
    HideForOtherTeams = 2,
    HideForOwnTeam = 3
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeamCollisionRule {
    Always = 0,
    Never = 1,
    HideForOtherTeams = 2,
    HideForOwnTeam = 3
}