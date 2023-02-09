use std::error::Error;

use serde::{Deserialize, Serialize};

use super::{player::Player, stats_json::StatsJson};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RoflJson {
    gameLength: i32,
    gameVersion: String,
    lastGameChunkId: i32,
    lastKeyFrameId: i32,
    statsJson: String,
}

impl RoflJson {
    pub fn get_stats_json(&self) -> Result<StatsJson, Box<dyn Error>> {
        let players: Vec<Player> = serde_json::from_str(&self.statsJson)?;
        Ok(StatsJson::new(players))
    }
}
