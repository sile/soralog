use crate::{
    jsonl,
    messages::{FieldName, Message},
};
use orfail::OrFail;

#[derive(Debug, clap::Args)]
pub struct WithCommand {
    pub fields: Vec<FieldName>,
}

impl WithCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let messages = jsonl::input_items::<Message>().map(|m| {
            m.and_then(|m| {
                let mut map = serde_json::Map::new();
                for &field in &self.fields {
                    if let Some(v) = m.get_field_value(field) {
                        map.insert(field.to_string(), v.to_json_value());
                    } else {
                        map.insert(field.to_string(), serde_json::Value::Null);
                    }
                }
                Ok(map)
            })
        });
        jsonl::output_items(messages).or_fail()?;
        Ok(())
    }
}
