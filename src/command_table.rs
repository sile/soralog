use crate::{jsonl, message::JsonMap};
use orfail::OrFail;
use std::collections::BTreeMap;

#[derive(Debug, clap::Args)]
pub struct TableCommand {}

impl TableCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut columns = Vec::<Column>::new();
        let mut messages = Vec::new();
        for m in jsonl::input_items::<JsonMap>() {
            let m = m.or_fail()?;
            for key in m.keys() {
                if columns.iter().any(|c| c.key == *key) {
                    continue;
                }
                columns.push(Column::new(key));
            }
            messages.push(
                m.into_iter()
                    .map(|(k, v)| (k, json_value_to_string(&v)))
                    .collect::<BTreeMap<_, _>>(),
            );
        }

        for message in &messages {
            for (key, value) in message {
                let Some(col) = columns.iter_mut().find(|c| c.key == *key) else {
                    unreachable!();
                };
                col.update_width(value);
            }
        }

        for col in &columns {
            print!("| {:<width$} ", col.key, width = col.width);
        }
        println!("|");

        for col in &columns {
            print!("|-{:-<width$}-", "-", width = col.width);
        }
        println!("|");

        let null = "".to_string();
        for message in messages {
            for col in &columns {
                let value = message.get(&col.key).unwrap_or(&null);
                print!("| {:<width$} ", value, width = col.width);
            }
            println!("|");
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Column {
    key: String,
    width: usize,
}

impl Column {
    fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
            width: key.len(),
        }
    }

    fn update_width(&mut self, value: &str) {
        self.width = self.width.max(value.len());
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
