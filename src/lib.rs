pub mod rofl;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::rofl::Rofl;

    #[test]
    fn parse_rofl() -> Result<(), Box<dyn Error>> {
        let mut rofl = Rofl::new();
        let parse_result = rofl.parse_rofl("KR-5736537983.rofl");

        if let Ok(_) = parse_result {
            let rofl_json = rofl.get_rofl_json();
            let stats_json = rofl.get_stats_json();
        }

        Ok(())
    }
}
