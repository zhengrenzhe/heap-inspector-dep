use std::convert::Infallible;
use std::fs;
use std::sync::Mutex;
use std::thread;

use log::{debug, error};
use serde_json::json;
use warp::{Filter, Reply};

use crate::analyzer::analyzer::{Analyzer, SearchQuery};
use crate::utils::browser::open_url;
use crate::utils::http::{json_err_res, json_ok_res};
use crate::utils::webpage::webpage_routes;

struct State {
    analyzer: Option<Analyzer>,
    is_ready: bool,
}
static STATE: Mutex<State> = Mutex::new(State {
    analyzer: None,
    is_ready: false,
});

pub struct Local {}

impl Local {
    pub fn new(file_path: &String) -> Self {
        let fp = file_path.clone();
        let fp2 = file_path.clone();

        thread::spawn(move || {
            debug!("start analyse file {}", fp);

            if let Ok(bytes) = fs::read(fp) {
                debug!("reading finished");
                debug!("start analyse");
                let analyzer = Analyzer::from_bytes(&bytes);
                let mut lock = STATE.lock().expect("get state lock error");
                lock.analyzer = Some(analyzer);
                lock.is_ready = true;
                debug!("analyse finished");
                return;
            }

            error!("{} not exist", fp2);
        });

        Local {}
    }

    pub async fn start(&self) {
        let port = portpicker::pick_unused_port().expect("No ports free");

        let routes = webpage_routes()
            .or(warp::path!("api" / "is_ready").and_then(Local::is_ready))
            .or(warp::path!("api" / "search")
                .and(warp::query::raw())
                .and_then(Local::search));

        let url = format!("http://localhost:{}", port);
        println!("✨ open {}", url);
        open_url(&url);
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    }

    pub async fn is_ready() -> Result<impl Reply, Infallible> {
        let is_ready = STATE.lock().unwrap().is_ready;
        json_ok_res(json!({ "is_ready": is_ready }))
    }

    pub async fn search(q: String) -> Result<impl Reply, Infallible> {
        let query = serde_qs::from_str::<SearchQuery>(&q).unwrap();
        let lock = STATE.lock().expect("get state lock error");
        if let Some(analyzer) = &lock.analyzer {
            analyzer.search(&query);
            return json_ok_res(json!({ "X":"X" }));
        }
        json_err_res(json!({ "error":true }))
    }
}
