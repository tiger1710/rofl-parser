use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Player {
    V13_3(super::player_13_3::Player),
    V12_3(super::player_12_3::Player),
}

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
