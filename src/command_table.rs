use crate::{json_stream, message::JsonMap};
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
        let mut rows = Vec::new();
        for message in json_stream::input_items::<JsonMap>() {
            let message = message.or_fail()?;
            let mut row = BTreeMap::new();
            for column in &mut columns {
                let mut value =
                    json_value_to_string(&message.get(&column.key).cloned().unwrap_or_default());
                if let Some(max_column_width) = self.max_column_width {
                    if value.len() > max_column_width {
                        value.truncate(max_column_width);
                        value.push_str("...");
                    }
                }

                column.update_width(&value);
                row.insert(column.key.clone(), value);
            }
            rows.push(row);
        }

        rows.sort_by(|x, y| {
            let xs = columns.iter().map(|c| x.get(&c.key));
            let ys = columns.iter().map(|c| y.get(&c.key));
            xs.cmp(ys)
        });

        for col in &columns {
            print!("| {:<width$} ", col.key, width = col.width);
        }
        println!("|");

        for col in &columns {
            print!("|-{:-<width$}-", "-", width = col.width);
        }
        println!("|");

        let null = "".to_string();
        for row in rows {
            for col in &columns {
                let value = row.get(&col.key).unwrap_or(&null);
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
