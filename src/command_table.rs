use crate::{jsonl, message::JsonMap};
use orfail::OrFail;
use std::collections::BTreeMap;

#[derive(Debug, clap::Args)]
pub struct TableCommand {
    #[clap(long, short)]
    pub max_column_width: Option<usize>,

    pub column_keys: Vec<String>,
}

impl TableCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut columns = self
            .column_keys
            .iter()
            .map(|key| Column::new(key))
            .collect::<Vec<_>>();
        let mut messages = Vec::new();
        for m in jsonl::input_items::<JsonMap>() {
            let m = m.or_fail()?;
            messages.push(
                m.into_iter()
                    .map(|(k, v)| {
                        let mut v = json_value_to_string(&v);
                        if let Some(max_column_width) = self.max_column_width {
                            if v.len() > max_column_width {
                                v.truncate(max_column_width);
                                v.push_str("...");
                            }
                        }
                        (k, v)
                    })
                    .collect::<BTreeMap<_, _>>(),
            );
        }

        for message in &messages {
            for (key, value) in message {
                let Some(col) = columns.iter_mut().find(|c| c.key == *key) else {
                    continue;
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
