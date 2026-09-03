//! Defines the criteria of objectives and their helpers.
use super::macros::{
    define_custom_vanilla_criteria, define_team_vanilla_criteria, vanilla_criterion,
};
use crate::scoreboard::team_color::TeamColor;
use rustc_hash::FxHashMap;
use std::sync::{LazyLock, OnceLock};
use steel_registry::stat::Stat;
use steel_registry::{REGISTRY, RegistryExt};
use steel_utils::Identifier;

static CRITERIA: LazyLock<ObjectiveCriteria> = LazyLock::new(ObjectiveCriteria::init);

define_custom_vanilla_criteria! {
    /// A criterion which does not update on its own. It is unlocked so that the objectives having
    /// this criterion can be modified in any arbitrary way.
    DUMMY("dummy"),

    /// A criterion whose scores can be modified when players execute the `/trigger` command.
    TRIGGER("trigger"),

    /// A criterion that increments score for a player each time they die.
    DEATH_COUNT("deathCount"),

    /// A criterion that increments score for a player each time they kill another player.
    KILL_COUNT_PLAYERS("playerKillCount"),

    /// A criterion that increments score for a player for each time they kill another entity.
    KILL_COUNT_ALL("totalKillCount"),

    /// A locked criterion that updates the score for a player to their health when that updates.
    HEALTH("health") => false, RenderType::Hearts,

    /// A locked criterion that updates the score for a player to their hunger level when that updates.
    FOOD("food") => false, RenderType::Integer,

    /// A locked criterion that updates the score for a player to their air level when that updates.
    AIR("air") => false, RenderType::Integer,

    /// A locked criterion that updates the score for a player to their armor level when that updates.
    ARMOR("armor") => false, RenderType::Integer,

    /// A locked criterion that updates the score for a player to their total experience points when that updates.
    EXPERIENCE("xp") => false, RenderType::Integer,

    /// A locked criterion that updates the score for a player to their experience level when that updates.
    LEVEL("level") => false, RenderType::Integer,
}

define_team_vanilla_criteria! {
    /// Updates when a player kills another entity from this team.
    TEAM_KILL("teamkill"),

    /// Updates when this player is killed by another entity from this team.
    KILLED_BY_TEAM("killedByTeam"),
}

/// Represents the criterion of an objective, which will update the scores in the objective
/// depending on the instance of the criterion used.
///
/// For example, the `health` criterion updates to the new value whenever a player's health changes.
#[derive(Debug)]
pub struct ObjectiveCriterion {
    name: &'static str,
    read_only: bool,
    render_type: RenderType,
}

impl ObjectiveCriterion {
    const DEFAULT_READ_ONLY: bool = false;
    const DEFAULT_RENDER_TYPE: RenderType = RenderType::Integer;

    const fn custom(name: &'static str) -> Self {
        Self::custom_with_properties(name, Self::DEFAULT_READ_ONLY, Self::DEFAULT_RENDER_TYPE)
    }

    const fn custom_with_properties(
        name: &'static str,
        read_only: bool,
        render_type: RenderType,
    ) -> Self {
        Self {
            name,
            read_only,
            render_type,
        }
    }

    /// Gets a criterion from its name.
    pub fn from_name(name: &str) -> Option<&'static Self> {
        CRITERIA.get(name)
    }

    /// Returns the name used by this criterion.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns whether the scores of the objectives with this criterion are read-only, i.e.
    /// cannot be modified by commands directly.
    pub fn read_only(&self) -> bool {
        self.read_only
    }

    /// Returns how objectives with this criterion are displayed by default.
    pub fn render_type(&self) -> RenderType {
        self.render_type
    }
}

/// Tells the client how to display an objective's scores in the tab list.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RenderType {
    /** Tells the client to display the score as an integer in the tab list. */
    Integer,

    /** Tells the client to display the score as hearts in the tab list. */
    Hearts,
}

/// Represents a map where each team color has its own criterion.
pub struct TeamObjectiveCriteria {
    prefix: &'static str,
    criteria: OnceLock<[ObjectiveCriterion; TeamColor::VALUES.len()]>,
}

impl TeamObjectiveCriteria {
    const fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            criteria: OnceLock::new(),
        }
    }

    fn get_or_init(&self) -> &[ObjectiveCriterion; TeamColor::VALUES.len()] {
        self.criteria.get_or_init(|| {
            std::array::from_fn(|i| {
                let color = TeamColor::VALUES[i];
                let name = format!("{}.{color}", self.prefix);
                let name = Box::leak(name.into_boxed_str());
                ObjectiveCriterion::custom(name)
            })
        })
    }

    fn register(&'static self, criteria: &mut ObjectiveCriteria) {
        for criterion in self.get_or_init() {
            criteria
                .vanilla_custom_criteria
                .insert(criterion.name, &criterion);
        }
    }

    /// Gets the criterion stored in this map from its given color.
    pub fn get(&self, color: TeamColor) -> &ObjectiveCriterion {
        &self.get_or_init()[color as usize]
    }
}

struct ObjectiveCriteria {
    vanilla_custom_criteria: FxHashMap<&'static str, &'static ObjectiveCriterion>,
    stat_criteria: FxHashMap<Stat, &'static ObjectiveCriterion>,
}

impl ObjectiveCriteria {
    fn init() -> ObjectiveCriteria {
        let mut criteria = ObjectiveCriteria {
            vanilla_custom_criteria: FxHashMap::default(),
            stat_criteria: FxHashMap::default(),
        };

        register_vanilla_custom_criteria(&mut criteria);

        criteria
    }

    fn get(&self, name: &str) -> Option<&ObjectiveCriterion> {
        if let Some(criterion) = self.vanilla_custom_criteria.get(name) {
            return Some(*criterion);
        }

        let (namespace, path) = name.split_once(':')?;
        let stat_type_identifier = Identifier::parse_by_separator(namespace, '.').ok()?;
        let stat_value_identifier = Identifier::parse_by_separator(path, '.').ok()?;
        let stat_type_entry = REGISTRY.stat_types.by_key(&stat_type_identifier)?;
        let stat_value_entry = stat_type_entry.value_from_key(&stat_value_identifier)?;
        let stat = Stat::from_erased(stat_type_entry, stat_value_entry);
        self.stat_criteria.get(&stat).copied()
    }
}
