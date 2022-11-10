fn main() -> Result<(), anyhow::Error> {
    let arguments = std::env::args().into_iter().collect::<Vec<_>>();

    let splitted = arguments
        .get(1)
        .unwrap()
        .split('.')
        .into_iter()
        .collect::<Vec<&str>>();

    decode_and_print(splitted[0])?;
    decode_and_print(splitted[1])?;

    Ok(())
}

fn decode_and_print(val: &str) -> Result<(), anyhow::Error> {
    let decoded = base64::decode(val)?;

    let s = serde_json::from_slice::<serde_json::Value>(&decoded)?;

    println!("{}", serde_json::to_string_pretty(&s)?);

    Ok(())
}
