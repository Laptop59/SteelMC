use std::collections::btree_map::Entry;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use text_components::TextComponent;
use thiserror::Error;
use steel_utils::locks::SyncRwLock;
use crate::scoreboard::criterion::{ObjectiveCriterion, RenderType};

/// Score holder name stored by the vanilla scoreboard.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScoreHolder {
    name: String,
}

impl ScoreHolder {
    /// Creates a score holder from its scoreboard name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the scoreboard name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Objective identity resolved from one domain scoreboard.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreboardObjective {
    name: String,
    read_only: bool,
}

impl ScoreboardObjective {
    /// Returns the objective name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether commands may change scores for this objective.
    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        self.read_only
    }
}

/// Team identity resolved from one domain scoreboard.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreboardTeam {
    name: String,
}

impl ScoreboardTeam {
    /// Returns the team name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Stored score fields used by command execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ScoreboardScore {
    value: i32,
    #[serde(default = "default_score_locked")]
    locked: bool,
}

impl ScoreboardScore {
    const fn new(value: i32) -> Self {
        Self {
            value,
            locked: true,
        }
    }

    /// Returns the integer score value.
    #[must_use]
    pub const fn value(self) -> i32 {
        self.value
    }

    /// Returns whether `/trigger`-style writes are locked.
    #[must_use]
    pub const fn is_locked(self) -> bool {
        self.locked
    }
}

impl Default for ScoreboardScore {
    fn default() -> Self {
        Self::new(0)
    }
}

const fn default_score_locked() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Objective {
    criterion: ObjectiveCriterion,
    display_name: TextComponent,
    render_type: RenderType,
}

/// Invalid scoreboard operation or persisted state.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ScoreboardError {
    /// Objective names may not be empty.
    #[error("objective name cannot be empty")]
    EmptyObjectiveName,
    /// Team names may not be empty.
    #[error("team name cannot be empty")]
    EmptyTeamName,
    /// Score holder names may not be empty.
    #[error("score holder name cannot be empty")]
    EmptyScoreHolderName,
    /// An objective already exists.
    #[error("objective '{0}' already exists")]
    DuplicateObjective(String),
    /// A team already exists.
    #[error("team '{0}' already exists")]
    DuplicateTeam(String),
    /// The requested objective does not exist.
    #[error("objective '{0}' does not exist")]
    MissingObjective(String),
    /// The requested team does not exist.
    #[error("team '{0}' does not exist")]
    MissingTeam(String),
    /// The objective cannot be written by commands.
    #[error("objective '{0}' is read-only")]
    ReadOnlyObjective(String),
}

struct ScoreboardSaveSnapshot {
    revision: u64,
    state: PersistentScoreboard,
}

/// Command-facing scoreboard for one Steel domain.
pub struct Scoreboard {
    state: SyncRwLock<PersistentScoreboard>,
    revision: AtomicU64,
    saved_revision: AtomicU64,
}

impl Scoreboard {
    /// Creates an empty, clean scoreboard.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: SyncRwLock::new(PersistentScoreboard::default()),
            revision: AtomicU64::new(0),
            saved_revision: AtomicU64::new(0),
        }
    }

    fn from_persistent(state: PersistentScoreboard) -> Result<Self, ScoreboardError> {
        validate_persistent_scoreboard(&state)?;
        Ok(Self {
            state: SyncRwLock::new(state),
            revision: AtomicU64::new(0),
            saved_revision: AtomicU64::new(0),
        })
    }

    /// Adds a writable objective.
    ///
    /// # Errors
    ///
    /// Returns an error if the objective name is empty or already exists.
    pub fn add_objective(
        &self,
        name: impl Into<String>,
    ) -> Result<ScoreboardObjective, ScoreboardError> {
        self.add_objective_with_read_only(name, false)
    }

    /// Adds an objective with explicit command mutability.
    ///
    /// # Errors
    ///
    /// Returns an error if the objective name is empty or already exists.
    pub fn add_objective_with_read_only(
        &self,
        name: impl Into<String>,
        read_only: bool,
    ) -> Result<ScoreboardObjective, ScoreboardError> {
        let name = name.into();
        ensure_objective_name(&name)?;
        let mut state = self.state.write();
        if state.objectives.contains_key(&name) {
            return Err(ScoreboardError::DuplicateObjective(name));
        }
        state
            .objectives
            .insert(name.clone(), Objective { read_only });
        self.mark_dirty();
        Ok(ScoreboardObjective { name, read_only })
    }

    /// Returns an objective by name.
    #[must_use]
    pub fn objective(&self, name: &str) -> Option<ScoreboardObjective> {
        self.state
            .read()
            .objectives
            .get(name)
            .map(|objective| ScoreboardObjective {
                name: name.to_owned(),
                read_only: objective.read_only,
            })
    }

    /// Returns objective names in stable order.
    #[must_use]
    pub fn objective_names(&self) -> Vec<String> {
        self.state.read().objectives.keys().cloned().collect()
    }

    /// Adds a team.
    ///
    /// # Errors
    ///
    /// Returns an error if the team name is empty or already exists.
    pub fn add_team(&self, name: impl Into<String>) -> Result<ScoreboardTeam, ScoreboardError> {
        let name = name.into();
        ensure_team_name(&name)?;
        let mut state = self.state.write();
        if !state.teams.insert(name.clone()) {
            return Err(ScoreboardError::DuplicateTeam(name));
        }
        self.mark_dirty();
        Ok(ScoreboardTeam { name })
    }

    /// Returns a team by name.
    #[must_use]
    pub fn team(&self, name: &str) -> Option<ScoreboardTeam> {
        self.state
            .read()
            .teams
            .contains(name)
            .then(|| ScoreboardTeam {
                name: name.to_owned(),
            })
    }

    /// Returns team names in stable order.
    #[must_use]
    pub fn team_names(&self) -> Vec<String> {
        self.state.read().teams.iter().cloned().collect()
    }

    /// Returns the current team name for a score holder.
    #[must_use]
    pub fn holder_team_name(&self, holder: &ScoreHolder) -> Option<String> {
        self.state.read().holder_teams.get(holder.name()).cloned()
    }

    /// Adds a holder to a team, replacing any prior membership.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty holder or a team that no longer exists.
    pub fn add_holder_to_team(
        &self,
        holder: &ScoreHolder,
        team: &ScoreboardTeam,
    ) -> Result<(), ScoreboardError> {
        ensure_holder_name(holder.name())?;
        let mut state = self.state.write();
        if !state.teams.contains(team.name()) {
            return Err(ScoreboardError::MissingTeam(team.name().to_owned()));
        }
        if state
            .holder_teams
            .insert(holder.name().to_owned(), team.name().to_owned())
            .as_deref()
            == Some(team.name())
        {
            return Ok(());
        }
        self.mark_dirty();
        Ok(())
    }

    /// Returns tracked score holders in stable order.
    #[must_use]
    pub fn tracked_holders(&self) -> Vec<ScoreHolder> {
        self.state
            .read()
            .scores
            .keys()
            .map(|name| ScoreHolder::new(name.to_owned()))
            .collect()
    }

    /// Returns the complete score entry for a holder and objective.
    #[must_use]
    pub fn score_entry(
        &self,
        holder: &ScoreHolder,
        objective: &ScoreboardObjective,
    ) -> Option<ScoreboardScore> {
        self.state
            .read()
            .scores
            .get(holder.name())
            .and_then(|scores| scores.get(objective.name()).copied())
    }

    /// Returns the integer score for a holder and objective.
    #[must_use]
    pub fn score(&self, holder: &ScoreHolder, objective: &ScoreboardObjective) -> Option<i32> {
        self.score_entry(holder, objective)
            .map(ScoreboardScore::value)
    }

    /// Sets a holder's score, preserving its lock state when already present.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty holder, a missing objective, or a read-only objective.
    pub fn set_score(
        &self,
        holder: &ScoreHolder,
        objective: &ScoreboardObjective,
        value: i32,
    ) -> Result<(), ScoreboardError> {
        ensure_holder_name(holder.name())?;
        let mut state = self.state.write();
        ensure_writable_objective(&state, objective)?;
        let scores = state.scores.entry(holder.name().to_owned()).or_default();
        match scores.entry(objective.name().to_owned()) {
            Entry::Vacant(entry) => {
                entry.insert(ScoreboardScore::new(value));
            }
            Entry::Occupied(mut entry) => {
                if entry.get().value == value {
                    return Ok(());
                }
                entry.get_mut().value = value;
            }
        }
        self.mark_dirty();
        Ok(())
    }

    /// Changes a score's trigger lock state.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty holder or a missing objective.
    pub fn set_score_locked(
        &self,
        holder: &ScoreHolder,
        objective: &ScoreboardObjective,
        locked: bool,
    ) -> Result<(), ScoreboardError> {
        ensure_holder_name(holder.name())?;
        let mut state = self.state.write();
        ensure_objective_exists(&state, objective)?;
        let scores = state.scores.entry(holder.name().to_owned()).or_default();
        match scores.entry(objective.name().to_owned()) {
            Entry::Vacant(entry) => {
                entry.insert(ScoreboardScore { value: 0, locked });
            }
            Entry::Occupied(mut entry) => {
                if entry.get().locked == locked {
                    return Ok(());
                }
                entry.get_mut().locked = locked;
            }
        }
        self.mark_dirty();
        Ok(())
    }

    /// Returns objective names that have a score for the holder.
    #[must_use]
    pub fn holder_objectives(&self, holder: &ScoreHolder) -> BTreeSet<String> {
        self.state
            .read()
            .scores
            .get(holder.name())
            .map_or_else(BTreeSet::new, |scores| scores.keys().cloned().collect())
    }

    fn mark_dirty(&self) {
        self.revision.fetch_add(1, Ordering::Release);
    }

    fn pending_save(&self) -> Option<ScoreboardSaveSnapshot> {
        let state = self.state.read();
        let revision = self.revision.load(Ordering::Acquire);
        if revision == self.saved_revision.load(Ordering::Acquire) {
            return None;
        }
        Some(ScoreboardSaveSnapshot {
            revision,
            state: state.clone(),
        })
    }

    fn mark_saved(&self, revision: u64) {
        self.saved_revision.fetch_max(revision, Ordering::Release);
    }
}

impl Default for Scoreboard {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_persistent_scoreboard(state: &PersistentScoreboard) -> Result<(), ScoreboardError> {
    for name in state.objectives.keys() {
        ensure_objective_name(name)?;
    }
    for name in &state.teams {
        ensure_team_name(name)?;
    }
    for (holder, scores) in &state.scores {
        ensure_holder_name(holder)?;
        for objective in scores.keys() {
            if !state.objectives.contains_key(objective) {
                return Err(ScoreboardError::MissingObjective(objective.clone()));
            }
        }
    }
    for (holder, team) in &state.holder_teams {
        ensure_holder_name(holder)?;
        if !state.teams.contains(team) {
            return Err(ScoreboardError::MissingTeam(team.clone()));
        }
    }
    Ok(())
}

const fn ensure_objective_name(name: &str) -> Result<(), ScoreboardError> {
    if name.is_empty() {
        Err(ScoreboardError::EmptyObjectiveName)
    } else {
        Ok(())
    }
}

const fn ensure_team_name(name: &str) -> Result<(), ScoreboardError> {
    if name.is_empty() {
        Err(ScoreboardError::EmptyTeamName)
    } else {
        Ok(())
    }
}

const fn ensure_holder_name(name: &str) -> Result<(), ScoreboardError> {
    if name.is_empty() {
        Err(ScoreboardError::EmptyScoreHolderName)
    } else {
        Ok(())
    }
}

fn ensure_objective_exists(
    state: &PersistentScoreboard,
    objective: &ScoreboardObjective,
) -> Result<Objective, ScoreboardError> {
    state
        .objectives
        .get(objective.name())
        .copied()
        .ok_or_else(|| ScoreboardError::MissingObjective(objective.name().to_owned()))
}

fn ensure_writable_objective(
    state: &PersistentScoreboard,
    objective: &ScoreboardObjective,
) -> Result<(), ScoreboardError> {
    if ensure_objective_exists(state, objective)?.read_only {
        Err(ScoreboardError::ReadOnlyObjective(
            objective.name().to_owned(),
        ))
    } else {
        Ok(())
    }
}