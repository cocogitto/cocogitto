use cocogitto_config::Settings;
use schemars::schema_for;

/**
Prints the json schema of [Settings] to stdout
## Example
```bash
cargo run > cog-schema.json
```
 */
fn main() {
    let schema = schema_for!(Settings);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
