use crate::jsonl;
use orfail::OrFail;

type Map = serde_json::Map<String, serde_json::Value>;

#[derive(Debug, clap::Args)]
pub struct TableCommand {}

impl TableCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let messages = jsonl::input_items::<Map>()
            .collect::<orfail::Result<Vec<_>>>()
            .or_fail()?;

        let mut keys = Vec::new();
        for message in &messages {
            for key in message.keys() {
                if keys.contains(key) {
                    continue;
                }
                keys.push(key.to_owned());
            }
        }

        // TODO: add padding to align width
        for key in &keys {
            print!("| {key} ");
        }
        println!("|");

        for _ in &keys {
            print!("|---");
        }
        println!("|");

        for message in messages {
            for key in &keys {
                let value = message
                    .get(key)
                    .map(json_value_to_string)
                    .unwrap_or_else(|| "".to_string());
                print!("| {value} ",);
            }
            println!("|");
        }

        Ok(())
    }
}

fn json_value_to_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::Null => "".to_string(),
        serde_json::Value::Bool(v) => v.to_string(),
        serde_json::Value::Number(v) => v.to_string(),
        serde_json::Value::String(v) => v.replace('|', "\\|"),
        serde_json::Value::Array(v) => {
            let v = v
                .iter()
                .map(|v| json_value_to_string(v))
                .collect::<Vec<_>>();
            v.join(".")
        }
        serde_json::Value::Object(_) => "<object>".to_string(),
    }
}
