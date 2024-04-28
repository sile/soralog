use crate::{
    jsonl,
    message::{FieldName, Message2},
};
use orfail::OrFail;
use std::collections::BTreeMap;

#[derive(Debug, clap::Args)]
pub struct CountCommand {
    pub fields: Vec<FieldName>,
}

impl CountCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut counter = Counter::new();
        for message in jsonl::input_items::<Message2>() {
            let message = message.or_fail()?;
            counter.increment(&mut self.fields.iter().copied(), &message);
        }
        jsonl::output_item(counter).or_fail()?;
        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
enum Counter {
    Value(usize),
    Children(BTreeMap<String, Self>),
}

impl Counter {
    fn new() -> Self {
        Self::Value(0)
    }

    fn increment(&mut self, fields: &mut impl Iterator<Item = FieldName>, message: &Message2) {
        let Some(field) = fields.next() else {
            match self {
                Self::Value(count) => {
                    *count += 1;
                }
                Self::Children(map) => {
                    map.entry("__OTHER__".to_string())
                        .or_insert_with(Self::new)
                        .increment(fields, message);
                }
            }
            return;
        };

        let Some(key) = message.get_field_value(field) else {
            self.increment(fields, message);
            return;
        };

        if let Self::Value(_) = self {
            *self = Self::Children(BTreeMap::new());
        }
        let Self::Children(children) = self else {
            unreachable!();
        };
        children
            .entry(key.to_string())
            .or_insert_with(Self::new)
            .increment(fields, message);
    }
}
