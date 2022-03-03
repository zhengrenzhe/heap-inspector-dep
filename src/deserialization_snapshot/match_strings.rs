use nom::bytes::complete::{tag, take_until};
use nom::IResult;

pub fn match_strings(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = take_until("strings")(input)?;
    let (input, _) = take_until("[")(input)?;
    let (input, _) = tag("[")(input)?;
    let input = input.strip_suffix("]}").unwrap();

    let strings: Vec<String> = input
        .split(",\n")
        .map(|val| {
            val.trim()
                .strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
                .to_string()
        })
        .collect();

    Ok(("", strings))
}
