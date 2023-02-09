
use serde::{Serialize, Deserialize};

use super::player::Player;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct StatsJson {
    statsJson: Vec<Player>
}

impl StatsJson {
    pub const fn new(players: Vec<Player>) -> StatsJson {
        StatsJson { statsJson: players }
    }
}
