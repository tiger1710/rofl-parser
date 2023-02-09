mod player;
pub mod rofl_json;
mod stats_json;


use std::{fs::File, io::Read, path::Path};

pub struct Rofl {
    rofl_string: String,
}

impl Rofl {
    pub const fn new() -> Rofl {
        Rofl { rofl_string: String::new()  }
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

    pub fn parse_rofl_to_string<P>(&mut self, file: P) -> &str
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
        
        self.rofl_string.as_str()
    }

}
