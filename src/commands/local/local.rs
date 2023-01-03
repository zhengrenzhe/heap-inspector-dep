use std::convert::Infallible;
use std::fs;
use std::sync::Mutex;
use std::thread;

use log::error;
use serde_json::json;
use spinach::Spinach;
use warp::{Filter, Reply};

use crate::analyzer::api::search::SearchQuery;
use crate::analyzer::Analyzer;
use crate::utils::browser::open_url;
use crate::utils::http::{json_err_res, json_ok_res};
use crate::utils::time_count::TimeCount;
use crate::utils::webpage::webpage_routes;

struct State {
    analyzer: Option<Analyzer>,
    is_ready: bool,
    file_path: String,
}
static STATE: Mutex<State> = Mutex::new(State {
    analyzer: None,
    is_ready: false,
    file_path: String::new(),
});

pub struct Local {
    port: u16,
}

impl Local {
    pub fn new(file_path: &String, port: &Option<u16>) -> Self {
        let fp = file_path.clone();
        let fp2 = file_path.clone();

        let port = match port {
            Some(port) => *port,
            None => 9999,
        };

        thread::spawn(move || {
            let progress = Spinach::new(format!("reading file {}", fp));

            let start = TimeCount::start();
            if let Ok(bytes) = fs::read(fp) {
                let diff = start.end();
                progress.succeed(format!("reading finished with {:?}", diff));

                let progress = Spinach::new("analysing...");
                let start = TimeCount::start();
                let analyzer = Analyzer::from_bytes(&bytes);

                let mut lock = STATE.lock().expect("get state lock error");
                lock.analyzer = Some(analyzer);
                lock.is_ready = true;
                lock.file_path = fp2.clone();

                let diff = start.end();
                progress.succeed(format!("analyse finished with {:?}", diff));

                let url = format!("http://localhost:{}", port);
                let progress = Spinach::new(format!("open {}", url));
                open_url(&url);
                progress.succeed(format!("open {}", url));

                return;
            }

            error!("{} not exist", fp2);
        });

        Local { port }
    }

    pub async fn start(&self) {
        let routes = webpage_routes()
            .or(warp::path!("api" / "is_ready").and_then(Local::is_ready))
            .or(warp::path!("api" / "meta").and_then(Local::meta))
            .or(warp::path!("api" / "statistics").and_then(Local::statistics))
            .or(warp::path!("api" / "constructors").and_then(Local::constructors))
            .or(warp::path!("api" / "search")
                .and(warp::query::raw())
                .and_then(Local::search));

        warp::serve(routes).run(([127, 0, 0, 1], self.port)).await;
    }

    pub async fn meta() -> Result<impl Reply, Infallible> {
        match &(STATE.lock()) {
            Ok(lock) => match &lock.analyzer {
                Some(analyzer) => {
                    let meta = analyzer.meta();
                    json_ok_res(json!({
                        "edge_count": meta.edge_count,
                        "node_count": meta.node_count,
                        "file_size": meta.file_size,
                        "file_path": lock.file_path,
                        "node_types": meta.node_types,
                        "edge_types": meta.edge_types
                    }))
                }
                None => json_err_res(json!({ "msg": "analyzer not found" })),
            },
            Err(_) => json_err_res(json!({ "msg": "get lock error" })),
        }
    }

    pub async fn is_ready() -> Result<impl Reply, Infallible> {
        match &(STATE.lock()) {
            Ok(lock) => json_ok_res(json!({"is_ready": lock.is_ready})),
            Err(_) => json_err_res(json!({ "msg": "analyzer not found" })),
        }
    }

    pub async fn search(q: String) -> Result<impl Reply, Infallible> {
        let query = serde_qs::from_str::<SearchQuery>(&q).unwrap();
        match &(STATE.lock()) {
            Ok(lock) => match &lock.analyzer {
                Some(analyzer) => json_ok_res(analyzer.search(&query)),
                None => json_err_res(json!({ "msg": "analyzer not found" })),
            },
            Err(_) => json_err_res(json!({ "msg": "get lock error" })),
        }
    }

    pub async fn statistics() -> Result<impl Reply, Infallible> {
        match &(STATE.lock()) {
            Ok(lock) => match &lock.analyzer {
                Some(analyzer) => json_ok_res(analyzer.statistics()),
                None => json_err_res(json!({ "msg": "analyzer not found" })),
            },
            Err(_) => json_err_res(json!({ "msg": "get lock error" })),
        }
    }

    pub async fn constructors() -> Result<impl Reply, Infallible> {
        match &(STATE.lock()) {
            Ok(lock) => match &lock.analyzer {
                Some(analyzer) => json_ok_res(json!(analyzer.constructors())),
                None => json_err_res(json!({ "msg": "analyzer not found" })),
            },
            Err(_) => json_err_res(json!({ "msg": "get lock error" })),
        }
    }
}
