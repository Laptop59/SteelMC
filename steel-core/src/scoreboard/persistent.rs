use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use text_components::TextComponent;
use crate::scoreboard::criterion::RenderType;
use crate::scoreboard::number_format::NumberFormat;
use crate::scoreboard::ScoreboardScore;
use crate::scoreboard::team::Team;



#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct PersistentObjective {
    criterion: &'static str,
    display_name: TextComponent,
    render_type: RenderType,
    display_auto_update: bool,
    number_format: Option<Box<dyn NumberFormat>>
}

impl PersistentObjective {
    fn from_objective()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct PersistentScoreboard {
    objectives: BTreeMap<Box<str>, PersistentObjective>,
    scores: BTreeMap<Box<str>, BTreeMap<Box<str>, ScoreboardScore>>,
    teams: BTreeMap<Box<str>, Team>,
    holder_teams: BTreeMap<Box<str>, Box<str>>,
}