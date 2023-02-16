pub mod player_12_3;
pub mod player_13_3;
pub mod rofl_json;

use std::{fs::File, io::Read, path::Path};

use self::rofl_json::{MockRoflJson, RoflJson};

pub struct Rofl {
    rofl_string: String,
    rofl_json: Option<RoflJson>,
}

impl Rofl {
    pub const fn new() -> Rofl {
        Rofl {
            rofl_string: String::new(),
            rofl_json: None,
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

    fn parse_rofl_to_string(&mut self, buf: &[u8]) {
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

    pub fn parse_rofl_file<P>(&mut self, rofl_file: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let buf = self.read_rofl(rofl_file);
        self.parse_rofl_to_string(&buf);

        let mock_rofl_json: MockRoflJson = serde_json::from_str(&self.rofl_string)?;
        let stats_json = mock_rofl_json.parse_stats_json()?;

        self.rofl_json = Some(RoflJson {
            gameLength: mock_rofl_json.gameLength,
            gameVersion: mock_rofl_json.gameVersion,
            lastGameChunkId: mock_rofl_json.lastGameChunkId,
            lastKeyFrameId: mock_rofl_json.lastKeyFrameId,
            statsJson: stats_json,
        });

        Ok(())
    }

    pub fn parse_rofl_data(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.parse_rofl_to_string(data);

        let mock_rofl_json: MockRoflJson = serde_json::from_str(&self.rofl_string)?;
        let stats_json = mock_rofl_json.parse_stats_json()?;

        self.rofl_json = Some(RoflJson {
            gameLength: mock_rofl_json.gameLength,
            gameVersion: mock_rofl_json.gameVersion,
            lastGameChunkId: mock_rofl_json.lastGameChunkId,
            lastKeyFrameId: mock_rofl_json.lastKeyFrameId,
            statsJson: stats_json,
        });

        Ok(())
    }

    pub fn get_rofl_json(&self) -> Option<&RoflJson> {
        self.rofl_json.as_ref()
    }
}
