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

pub fn output_items<T, I>(results: I) -> orfail::Result<()>
where
    T: serde::Serialize,
    I: Iterator<Item = orfail::Result<T>>,
{
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for result in results {
        let item = result.or_fail()?;
        let json = serde_json::to_string(&item).or_fail()?;
        if ignore_broken_pipe(writeln!(&mut stdout, "{}", json)).or_fail()? {
            break;
        }
    }
    Ok(())
}

pub fn output_item_pp<T>(item: T) -> orfail::Result<()>
where
    T: serde::Serialize,
{
    output_items_pp(std::iter::once(Ok(item))).or_fail()
}

pub fn output_items_pp<T, I>(results: I) -> orfail::Result<()>
where
    T: serde::Serialize,
    I: Iterator<Item = orfail::Result<T>>,
{
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for result in results {
        let item = result.or_fail()?;
        let json = serde_json::to_string_pretty(&item).or_fail()?;
        if ignore_broken_pipe(writeln!(&mut stdout, "{}", json)).or_fail()? {
            break;
        }
    }
    Ok(())
}

fn ignore_broken_pipe(result: std::io::Result<()>) -> std::io::Result<bool> {
    match result {
        Ok(()) => Ok(false),
        Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => Ok(true),
        Err(err) => Err(err),
    }
}
