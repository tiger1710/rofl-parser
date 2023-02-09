pub mod rofl;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::rofl::{Rofl, rofl_json::RoflJson};

    #[test]
    fn parse_rofl() -> Result<(), Box<dyn Error>> {
        let mut rofl = Rofl::new();
        let str = rofl.parse_rofl_to_string("KR-5736537983.rofl");

        let rofl_json: RoflJson = serde_json::from_str(str)?;
        let _ = rofl_json.get_stats_json()?;

        Ok(())
    }
}
