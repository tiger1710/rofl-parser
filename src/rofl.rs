mod player;
pub mod rofl_json;
mod stats_json;

use std::{error::Error, fs::File, io::Read, path::Path};

use self::{rofl_json::RoflJson, stats_json::StatsJson};

pub struct Rofl {
    rofl_string: String,
    rofl_json: Option<RoflJson>,
    stats_json: Option<StatsJson>,
}

impl Rofl {
    pub const fn new() -> Rofl {
        Rofl {
            rofl_string: String::new(),
            rofl_json: None,
            stats_json: None,
        }
    }

    fn read_rofl<P>(&self, file: P) -> Vec<u8>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(file).expect("Can't open file.");
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).expect("Can't read file.");
        buf
    }

    fn parse_rofl_to_string<P>(&mut self, file: P)
    where
        P: AsRef<Path>,
    {
        let buf = self.read_rofl(file);
        let start = b"{\"gameLength\"";
        let end = b"\"}]\"}";

        let start_pos = buf
            .windows(start.len())
            .position(|w| w == start)
            .expect("Can't find start position.");

        let end_pos = buf
            .windows(end.len())
            .position(|w| w == end)
            .expect("Can't find end position.");

        self.rofl_string = std::str::from_utf8(&buf[start_pos..(end_pos + end.len())])
            .expect("Can't parse rofl to string.")
            .to_string();
    }

    pub fn parse_rofl<P>(&mut self, rofl_file: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        self.parse_rofl_to_string(rofl_file);
        self.rofl_json = serde_json::from_str(&self.rofl_string)?;
        Ok(())
    }

    pub fn get_rofl_json(&self) -> &RoflJson {
        self.rofl_json.as_ref().expect("rofl_json is None.")
    }

    pub fn get_stats_json(&self) -> &StatsJson {
        self.stats_json.as_ref().expect("stats_json is None.")
    }
}
