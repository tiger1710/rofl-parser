use std::error::Error;

use serde::{Deserialize, Serialize};

use super::{player::Player, stats_json::StatsJson};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoflJson {
    pub gameLength: i32,
    pub gameVersion: String,
    pub lastGameChunkId: i32,
    pub lastKeyFrameId: i32,
    statsJson: String,
}

impl RoflJson {
    pub fn parse_stats_json(&self) -> Result<StatsJson, Box<dyn Error>> {
        let players: Vec<Player> = serde_json::from_str(&self.statsJson)?;
        Ok(StatsJson::new(players))
    }
}
