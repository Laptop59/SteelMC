use std::{
    env::temp_dir,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::fs;

use steel_utils::saved_data::{SavedDataManager, names as saved_data_names};

use crate::scoreboard::{
    AsyncMutex, DomainScoreboards, PersistentScoreboard, ScoreHolder, Scoreboard, ScoreboardError,
};

#[test]
fn score_value_and_lock_state_are_independent() {
    let scoreboard = Scoreboard::new();
    let objective = scoreboard
        .add_objective("kills")
        .expect("objective should be added");
    let holder = ScoreHolder::new("Steve");

    scoreboard
        .set_score(&holder, &objective, 7)
        .expect("score should be writable");
    scoreboard
        .set_score_locked(&holder, &objective, false)
        .expect("score lock should change");
    scoreboard
        .set_score(&holder, &objective, 9)
        .expect("score should remain writable");

    let entry = scoreboard
        .score_entry(&holder, &objective)
        .expect("score should exist");
    assert_eq!(entry.value(), 9);
    assert!(!entry.is_locked());
}

#[test]
fn read_only_objective_rejects_score_writes() {
    let scoreboard = Scoreboard::new();
    let objective = scoreboard
        .add_objective_with_read_only("health", true)
        .expect("objective should be added");

    assert_eq!(
        scoreboard.set_score(&ScoreHolder::new("Steve"), &objective, 20),
        Err(ScoreboardError::ReadOnlyObjective("health".to_owned()))
    );
}

#[test]
fn team_assignment_replaces_prior_membership() {
    let scoreboard = Scoreboard::new();
    let red = scoreboard
        .add_team("red")
        .expect("red team should be added");
    let blue = scoreboard
        .add_team("blue")
        .expect("blue team should be added");
    let holder = ScoreHolder::new("Steve");

    scoreboard
        .add_holder_to_team(&holder, &red)
        .expect("holder should join red");
    scoreboard
        .add_holder_to_team(&holder, &blue)
        .expect("holder should move to blue");

    assert_eq!(
        scoreboard.holder_team_name(&holder).as_deref(),
        Some("blue")
    );
}

#[tokio::test]
async fn persisted_scoreboard_round_trips_and_becomes_clean_after_save() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after Unix epoch")
        .as_nanos();
    let path = temp_dir().join(format!("steel-scoreboard-{unique}"));
    let manager = SavedDataManager::new(Some(&path));
    let scoreboard = Scoreboard::new();
    let objective = scoreboard
        .add_objective("kills")
        .expect("objective should be added");
    let holder = ScoreHolder::new("Steve");
    scoreboard
        .set_score(&holder, &objective, 5)
        .expect("score should be writable");

    let snapshot = scoreboard
        .pending_save()
        .expect("scoreboard should be dirty");
    manager
        .save(saved_data_names::SCOREBOARD, &snapshot.state)
        .await
        .expect("scoreboard should save");
    scoreboard.mark_saved(snapshot.revision);
    assert!(scoreboard.pending_save().is_none());

    let persistent: PersistentScoreboard = manager
        .load_or_default(saved_data_names::SCOREBOARD)
        .await
        .expect("scoreboard should load");
    let restored = Scoreboard::from_persistent(persistent).expect("scoreboard should validate");
    let restored_objective = restored
        .objective("kills")
        .expect("objective should persist");
    assert_eq!(restored.score(&holder, &restored_objective), Some(5));

    fs::remove_dir_all(path)
        .await
        .expect("temporary scoreboard directory should be removed");
}

#[test]
fn mutation_after_snapshot_remains_dirty_when_snapshot_is_marked_saved() {
    let scoreboard = Scoreboard::new();
    scoreboard
        .add_objective("kills")
        .expect("objective should be added");
    let snapshot = scoreboard
        .pending_save()
        .expect("scoreboard should be dirty");

    scoreboard
        .add_objective("deaths")
        .expect("second objective should be added");
    scoreboard.mark_saved(snapshot.revision);

    let pending = scoreboard
        .pending_save()
        .expect("newer mutation should remain dirty");
    assert!(pending.revision > snapshot.revision);
    assert!(pending.state.objectives.contains_key("deaths"));
}

#[test]
fn domains_keep_independent_scoreboards() {
    let scoreboards = DomainScoreboards {
        scoreboards: [
            ("alpha".to_owned(), Scoreboard::new()),
            ("beta".to_owned(), Scoreboard::new()),
        ]
        .into_iter()
        .collect(),
        save_lock: AsyncMutex::new(()),
    };
    scoreboards
        .get("alpha")
        .expect("alpha scoreboard should exist")
        .add_objective("kills")
        .expect("alpha objective should be added");

    assert!(
        scoreboards
            .get("beta")
            .expect("beta scoreboard should exist")
            .objective("kills")
            .is_none()
    );
}
