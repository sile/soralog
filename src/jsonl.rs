use orfail::OrFail;
use std::io::Write;

pub fn input_items<T>() -> impl Iterator<Item = orfail::Result<T>>
where
    T: serde::de::DeserializeOwned,
{
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    serde_json::Deserializer::from_reader(stdin)
        .into_iter::<T>()
        .map(|result| result.or_fail())
}

pub fn output_item<T>(item: T) -> orfail::Result<()>
where
    T: serde::Serialize,
{
    output_items(std::iter::once(Ok(item))).or_fail()
}

pub fn output_items<T, I>(results: I) -> orfail::Result<()>
where
    T: serde::Serialize,
    I: Iterator<Item = orfail::Result<T>>,
{
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for result in results {
        let item = result.or_fail()?;
        serde_json::to_writer(&mut stdout, &item).or_fail()?;
        writeln!(&mut stdout).or_fail()?;
    }
    Ok(())
}
