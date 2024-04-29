use crate::{json_stream, message::Message};
use orfail::OrFail;
use std::collections::BTreeMap;

/// ログメッセージ群を標準入力から受け取り、指定されたフィールドの値の出現回数をカウントします
#[derive(Debug, clap::Args)]
pub struct CountCommand {
    /// カウント対象のフィールド名（複数指定時にはその分だけ出力オブジェクトの階層が深くなる）
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
            let Self::Value(count) = self else {
                unreachable!();
            };
            *count += 1;
            return;
        };

        let key = message
            .get_value_string(field)
            .unwrap_or_else(|| "_OTHER_".to_string());

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
