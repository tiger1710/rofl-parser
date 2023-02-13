use serde::{Deserialize, Serialize};

use super::player::Player;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoflJson {
    pub gameLength: i32,
    pub gameVersion: String,
    pub lastGameChunkId: i32,
    pub lastKeyFrameId: i32,
    pub statsJson: Vec<Player>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MockRoflJson {
    pub gameLength: i32,
    pub gameVersion: String,
    pub lastGameChunkId: i32,
    pub lastKeyFrameId: i32,
    statsJson: String,
}

impl MockRoflJson {
    pub fn parse_stats_json(&self) -> anyhow::Result<Vec<Player>> {
        Ok(serde_json::from_str(&self.statsJson)?)
    }
}
