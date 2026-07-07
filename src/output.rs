use color_eyre::Result;

pub fn print_json<T: serde::Serialize>(data: &T, raw: bool) -> Result<()> {
    if raw {
        println!("{}", serde_json::to_string(data)?);
    } else {
        println!("{}", serde_json::to_string_pretty(data)?);
    }
    Ok(())
}
