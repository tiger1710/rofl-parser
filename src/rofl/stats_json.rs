use serde::{Deserialize, Serialize};

use super::player::Player;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatsJson {
    pub statsJson: Vec<Player>,
}

impl StatsJson {
    pub const fn new(players: Vec<Player>) -> StatsJson {
        StatsJson { statsJson: players }
    }
}
