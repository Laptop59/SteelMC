//! Domain-scoped command scoreboard state.
//!
//! This module owns the scoreboard data needed by selectors and command
//! execution: objective identity and mutability, score values and locks, and
//! team membership. Display slots and client presentation are outside this
//! command-system scope.

use std::{
    collections::{BTreeMap, BTreeSet, btree_map::Entry},
    io,
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Deserialize, Serialize};
use steel_utils::locks::{AsyncMutex, SyncRwLock};
use text_components::TextComponent;
use thiserror::Error;

use crate::scoreboard::criterion::{ObjectiveCriterion, RenderType};
use crate::{server::worlds::WorldMap, world::World};
use steel_utils::saved_data::names as saved_data_names;

pub mod criterion;
pub mod display_slot;
mod macros;
pub mod number_format;
pub mod team_color;
mod persistent;

/// Loaded command scoreboards keyed by Steel domain.
pub struct DomainScoreboards {
    scoreboards: BTreeMap<String, Scoreboard>,
    save_lock: AsyncMutex<()>,
}

impl DomainScoreboards {
    /// Loads one scoreboard through each domain's default world saved-data boundary.
    pub async fn load(worlds: &WorldMap) -> io::Result<Self> {
        let mut domains = worlds.domain_names().collect::<Vec<_>>();
        domains.sort_unstable();
        let mut scoreboards = BTreeMap::new();
        for domain in domains {
            let world = domain_default_world(worlds, domain)?;
            let persistent: PersistentScoreboard = world
                .saved_data
                .load_or_default(saved_data_names::SCOREBOARD)
                .await
                .map_err(|error| scoreboard_io_error(domain, error))?;
            let scoreboard = Scoreboard::from_persistent(persistent).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid scoreboard for domain '{domain}': {error}"),
                )
            })?;
            scoreboards.insert(domain.to_owned(), scoreboard);
        }
        Ok(Self {
            scoreboards,
            save_lock: AsyncMutex::new(()),
        })
    }

    /// Returns the scoreboard for a domain.
    #[must_use]
    pub fn get(&self, domain: &str) -> Option<&Scoreboard> {
        self.scoreboards.get(domain)
    }

    /// Saves every dirty domain scoreboard and returns the number written.
    pub async fn save(&self, worlds: &WorldMap) -> io::Result<usize> {
        let _save_guard = self.save_lock.lock().await;
        let mut saved = 0;
        for (domain, scoreboard) in &self.scoreboards {
            let Some(snapshot) = scoreboard.pending_save() else {
                continue;
            };
            let world = domain_default_world(worlds, domain)?;
            world
                .saved_data
                .save(saved_data_names::SCOREBOARD, &snapshot.state)
                .await
                .map_err(|error| scoreboard_io_error(domain, error))?;
            scoreboard.mark_saved(snapshot.revision);
            saved += 1;
        }
        Ok(saved)
    }
}

fn domain_default_world<'a>(worlds: &'a WorldMap, domain: &str) -> io::Result<&'a World> {
    worlds
        .default_world(domain)
        .map(AsRef::as_ref)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("domain '{domain}' has no loaded default world"),
            )
        })
}

fn scoreboard_io_error(domain: &str, error: io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("scoreboard I/O failed for domain '{domain}': {error}"),
    )
}

#[cfg(test)]
mod tests;
mod scoreboard;
mod team;
mod score;