use nom::bytes::complete::{tag, take_until};
use nom::character::complete::one_of;
use nom::multi::many0;
use nom::IResult;

pub fn trim_start_space(input: &str) -> IResult<&str, &str> {
    let (input, _) = many0(one_of(" \n"))(input)?;
    Ok((input, "trim_start_space"))
}

pub fn enter_bracket(input: &str) -> IResult<&str, &str> {
    let (input, _) = trim_start_space(input)?;
    let (input, _) = tag("{")(input)?;
    Ok((input, "access_child"))
}

pub fn get_key(input: &str) -> IResult<&str, &str> {
    let (input, _) = trim_start_space(input)?;
    let (input, _) = tag("\"")(input)?;
    let (_, input) = take_until("\"")(input)?;
    Ok((input, "get_key"))
}

pub fn split_u32_array(str: &str) -> Vec<u32> {
    str.split(',')
        .map(|val| val.trim().parse::<u32>().unwrap())
        .collect()
}

pub fn split_array(str: &str) -> Vec<String> {
    str.split(',')
        .map(|field| {
            field
                .trim()
                .strip_prefix('"')
                .unwrap()
                .trim()
                .strip_suffix('"')
                .unwrap()
                .trim()
                .to_string()
        })
        .collect()
}

#[test]
fn test_split_u32_array() {
    assert_eq!(
        split_u32_array("11840,169377,32,6,0,0\n"),
        vec![11840, 169377, 32, 6, 0, 0]
    );
    assert_eq!(
        split_u32_array("  11840  , \n\n 169377\n,\n32,6,  0,0\n"),
        vec![11840, 169377, 32, 6, 0, 0]
    );
}

#[test]
fn test_split_array() {
    assert_eq!(
        split_array("\"a\", \"b   b\", \"c\""),
        vec!["a", "b   b", "c"]
    );
    assert_eq!(split_array("\"a\", \"b\", \"c\""), vec!["a", "b", "c"]);
    assert_eq!(split_array("\"a\",\"b\",\"c\""), vec!["a", "b", "c"]);
    assert_eq!(
        split_array("\" a \", \"  b   \",   \" c\""),
        vec!["a", "b", "c"]
    );
}

#[test]
fn test_get_key() {
    assert_eq!(get_key("\"foo\""), Ok(("foo", "get_key")));
    assert_eq!(get_key("\"foo\"bar"), Ok(("foo", "get_key")));
    assert_eq!(get_key("    \"foo\"bar"), Ok(("foo", "get_key")));
}

#[test]
fn test_access_child() {
    assert_eq!(enter_bracket("{}"), Ok(("}", "access_child")));
    assert_eq!(
        enter_bracket("{\"\":\"\"}"),
        Ok(("\"\":\"\"}", "access_child"))
    );
}

#[test]
fn test_trim_start_space() {
    assert_eq!(trim_start_space(""), Ok(("", "trim_start_space")));
    assert_eq!(trim_start_space(" {"), Ok(("{", "trim_start_space")));
    assert_eq!(trim_start_space("        1"), Ok(("1", "trim_start_space")));
    assert_eq!(
        trim_start_space(
            r#"
                    {}"#
        ),
        Ok(("{}", "trim_start_space"))
    );
    assert_eq!(
        trim_start_space(
            r#"


                      {}"#
        ),
        Ok(("{}", "trim_start_space"))
    );
}
