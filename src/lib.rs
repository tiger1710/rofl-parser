pub mod rofl;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::rofl::Rofl;

    #[test]
    fn only_new() {
        let rofl = Rofl::new();

        assert!(rofl.get_rofl_json().is_none());
    }

    #[test]
    fn parse_rofl_file() -> anyhow::Result<()> {
        let mut rofl = Rofl::new();
        let _ = rofl.parse_rofl_file("KR-5736537983.rofl")?;

        let rofl_json = rofl.get_rofl_json();

        if let Some(rofl_json) = rofl_json {
            let parsed = serde_json::to_string_pretty(rofl_json)?;
            let parsed: String = parsed.split_whitespace().collect();

            let mut f = File::open("rofl.json").expect("Can't open file.");
            let mut buf = String::new();
            f.read_to_string(&mut buf).expect("Can't read file.");
            let buf: String = buf.split_whitespace().collect();

            assert_eq!(parsed, buf);
        }

        assert!(rofl_json.is_some());

        Ok(())
    }
}
