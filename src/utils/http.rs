use std::convert::Infallible;

use serde_json::Value;
use warp::http::response::Builder;
use warp::Reply;

pub fn json_res(val: Value) -> Result<impl Reply, Infallible> {
    return Ok(Builder::new()
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(200)
        .body(val.to_string())
        .unwrap());
}
