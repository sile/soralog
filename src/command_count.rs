use crate::{json_stream, message::Message};
use orfail::OrFail;
use std::collections::BTreeMap;

#[derive(Debug, clap::Args)]
pub struct CountCommand {
    pub keys: Vec<String>,
}

impl CountCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut counter = Counter::new();
        for message in json_stream::input_items::<Message>() {
            let message = message.or_fail()?;
            counter.increment(&mut self.keys.iter(), &message);
        }
        json_stream::output_item_pp(counter).or_fail()?;
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

    fn increment<'a>(
        &mut self,
        fields: &'a mut impl Iterator<Item = &'a String>,
        message: &Message,
    ) {
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

        let Some(key) = message.get_value_string(field) else {
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
