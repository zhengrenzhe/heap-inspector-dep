use std::convert::Infallible;
use std::sync::Mutex;
use std::thread;

use serde_json::json;
use spinach::Spinach;
use warp::{Filter, Reply};

use crate::analytics::provider::{
    ConstructorQuery, LocalConfig, Provider, ProviderMode, SearchQuery,
};
use crate::utils::browser::open_url;
use crate::utils::http::{json_err_res, json_ok_res};
use crate::utils::time_count::TimeCount;
use crate::utils::webpage::webpage_routes;

struct State {
    provider: Option<Provider>,
    is_ready: bool,
}

static STATE: Mutex<State> = Mutex::new(State {
    provider: None,
    is_ready: false,
});

pub struct Local {
    port: u16,
}

impl Local {
    pub fn new(file_path: &str, port: &Option<u16>) -> Self {
        let port = match port {
            Some(port) => *port,
            None => 9999,
        };

        let file_path = String::from(file_path);

        thread::spawn(move || {
            let progress = Spinach::new("reading and analyzing...");
            let start = TimeCount::start();

            let provider = Provider::new(ProviderMode::Local(LocalConfig { file_path }));

            let time_diff = start.end();
            progress.succeed(format!("analyse finished with {time_diff:?}"));

            let mut lock = STATE.lock().expect("get state lock error");
            lock.provider = Some(provider);
            lock.is_ready = true;

            let url = format!("http://localhost:{port}");
            open_url(&url);
        });

        Local { port }
    }

    pub async fn start(&self) {
        let routes = webpage_routes()
            .or(warp::path!("api" / "is_ready").and_then(Local::is_ready))
            .or(warp::path!("api" / "meta").and_then(Local::meta))
            .or(warp::path!("api" / "statistics").and_then(Local::statistics))
            .or(warp::path!("api" / "constructors")
                .and(warp::query::raw())
                .and_then(Local::constructors))
            .or(warp::path!("api" / "search")
                .and(warp::query::raw())
                .and_then(Local::search));

        warp::serve(routes).run(([127, 0, 0, 1], self.port)).await;
    }

    pub async fn meta() -> Result<impl Reply, Infallible> {
        match &(STATE.lock()) {
            Ok(lock) => match &lock.provider {
                Some(analyzer) => {
                    let meta = analyzer.meta();
                    json_ok_res(json!({
                        "edge_count": meta.edge_count,
                        "node_count": meta.node_count,
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
            Ok(lock) => match &lock.provider {
                Some(analyzer) => json_ok_res(analyzer.search(&query)),
                None => json_err_res(json!({ "msg": "analyzer not found" })),
            },
            Err(_) => json_err_res(json!({ "msg": "get lock error" })),
        }
    }

    pub async fn statistics() -> Result<impl Reply, Infallible> {
        match &(STATE.lock()) {
            Ok(lock) => match &lock.provider {
                Some(analyzer) => json_ok_res(analyzer.statistics()),
                None => json_err_res(json!({ "msg": "analyzer not found" })),
            },
            Err(_) => json_err_res(json!({ "msg": "get lock error" })),
        }
    }

    pub async fn constructors(q: String) -> Result<impl Reply, Infallible> {
        let query = serde_qs::from_str::<ConstructorQuery>(&q).unwrap();
        match &(STATE.lock()) {
            Ok(lock) => match &lock.provider {
                Some(analyzer) => json_ok_res(json!(analyzer.constructors(&query))),
                None => json_err_res(json!({ "msg": "analyzer not found" })),
            },
            Err(_) => json_err_res(json!({ "msg": "get lock error" })),
        }
    }
}
