# rofl-parser
Parse useable data from `*.rofl` file

# Usage

## 1. Parse form file

```rust
fn parse_rofl_file() -> Result<(), Box<dyn Error>> {
    let mut rofl = Rofl::new();
    let _ = rofl.parse_rofl_file("*.rofl")?; // rofl file download needed

    let rofl_json = rofl.get_rofl_json(); // get full rofl info
    let stats_json = rofl.get_stats_json(); // get player data

    assert!(rofl_json.is_some());
    assert!(stats_json.is_some());

    Ok(())
}
```

## 2. Parse from data

If we have `[u8]` data, parse from data

```rust
fn parse_rofl_file(data: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut rofl = Rofl::new();
    let _ = rofl.parse_rofl_data(data)?;

    let rofl_json = rofl.get_rofl_json();
    let stats_json = rofl.get_stats_json();

    assert!(rofl_json.is_some());
    assert!(stats_json.is_some());

    Ok(())
}
```