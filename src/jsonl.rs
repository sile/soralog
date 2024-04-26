use orfail::OrFail;
use std::io::Write;

pub fn output_jsonl<T, I>(items: I) -> orfail::Result<()>
where
    T: serde::Serialize,
    I: Iterator<Item = T>,
{
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for item in items {
        serde_json::to_writer(&mut stdout, &item).or_fail()?;
        writeln!(&mut stdout).or_fail()?;
    }
    Ok(())
}
