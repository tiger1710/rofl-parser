pub mod rofl;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::rofl::Rofl;

    #[test]
    fn only_new() {
        let rofl = Rofl::new();

        assert!(rofl.get_rofl_json().is_none());
        assert!(rofl.get_stats_json().is_none());
    }

    #[test]
    fn parse_rofl_file() -> Result<(), Box<dyn Error>> {
        let mut rofl = Rofl::new();
        let _ = rofl.parse_rofl_file("KR-5736537983.rofl")?;

        let rofl_json = rofl.get_rofl_json();
        let stats_json = rofl.get_stats_json();

        assert!(rofl_json.is_some());
        assert!(stats_json.is_some());

        Ok(())
    }
}
