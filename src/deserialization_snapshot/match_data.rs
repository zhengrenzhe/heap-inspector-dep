use nom::bytes::complete::{tag, take_until};
use nom::IResult;

use crate::deserialization_snapshot::utils::split_u32_array;

// got data
pub fn match_data<'a>(input: &'a str, field: &'a str) -> IResult<&'a str, Vec<u32>> {
    let (input, _) = take_until(field)(input)?;
    let (input, data) = take_until("]")(input)?;
    let (data, _) = take_until("[")(data)?;
    let (data, _) = tag("[")(data)?;
    let data = split_u32_array(data);
    Ok((input, data))
}
