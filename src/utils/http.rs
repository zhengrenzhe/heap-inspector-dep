use std::convert::Infallible;

use serde_json::Value;
use warp::http::response::Builder;
use warp::http::{Response, StatusCode};

fn json_res(val: Value, status: StatusCode) -> Result<Response<String>, Infallible> {
    Ok(Builder::new()
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(status)
        .body(val.to_string())
        .expect("build json_res error"))
}

pub fn json_ok_res(val: Value) -> Result<Response<String>, Infallible> {
    json_res(val, StatusCode::OK)
}

pub fn json_err_res(val: Value) -> Result<Response<String>, Infallible> {
    json_res(val, StatusCode::INTERNAL_SERVER_ERROR)
}
