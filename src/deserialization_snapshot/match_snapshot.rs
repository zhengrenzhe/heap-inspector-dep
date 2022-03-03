use nom::bytes::complete::{tag, take_until};
use nom::IResult;

use crate::deserialization_snapshot::utils::{get_key, split_array};

pub type SnapshotMeta = (Vec<String>, Vec<String>, u32, u32);

// got node_types and edge_types
pub fn match_snapshot(input: &str) -> IResult<&str, SnapshotMeta> {
    let (key, _) = get_key(input)?;
    if key != "snapshot" {
        panic!("current is not snapshot block");
    }

    // enter node_types
    let (input, _) = take_until("node_types")(input)?;
    // enter node_types left [
    let (input, _) = take_until("[")(input)?;
    // got node_types, end of right ]
    let (input, node_types) = take_until("edge_fields")(input)?;
    // eat left [[
    let (node_types, _) = tag("[[")(node_types)?;
    // end of first array
    let (_, node_types) = take_until("]")(node_types)?;
    let node_types = split_array(node_types);

    //  enter edge_types
    let (input, _) = take_until("edge_types")(input)?;
    // enter edge_types left [
    let (input, _) = take_until("[")(input)?;
    // got node_types, end of right ]
    let (input, edge_types) = take_until("trace_function_info_fields")(input)?;
    // eat left [[
    let (edge_types, _) = tag("[[")(edge_types)?;
    // end of first array
    let (_, edge_types) = take_until("]")(edge_types)?;
    let edge_types = split_array(edge_types);

    // node count
    let (input, _) = take_until("node_count")(input)?;
    let (input, node_count) = take_until(",")(input)?;
    let (node_count, _) = take_until(":")(node_count)?;
    let (node_count, _) = tag(":")(node_count)?;
    let node_count = node_count.parse::<u32>().unwrap();

    // edge count
    let (input, _) = take_until("edge_count")(input)?;
    let (input, edge_count) = take_until(",")(input)?;
    let (edge_count, _) = take_until(":")(edge_count)?;
    let (edge_count, _) = tag(":")(edge_count)?;
    let edge_count = edge_count.parse::<u32>().unwrap();

    Ok((input, (node_types, edge_types, node_count, edge_count)))
}
