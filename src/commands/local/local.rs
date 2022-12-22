use std::convert::Infallible;
use std::fs;
use std::sync::Mutex;
use std::thread;

use log::{debug, error};
use warp::http::StatusCode;
use warp::{Filter, Reply};

use crate::analyzer::analyzer::Analyzer;
use crate::utils::browser::open_url;
use crate::utils::webpage::webpage_routes;

struct State {
    analyzer: Option<Analyzer>,
    reading_done: bool,
}
static STATE: Mutex<State> = Mutex::new(State {
    analyzer: None,
    reading_done: false,
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
                lock.reading_done = true;
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
            .or(warp::path!("api" / "load_progress").and_then(|| Local::load_progress()))
            .or(warp::path!("api" / "toggle_lock").and_then(Local::toggle_lock));

        open_url(&format!("http://localhost:{}", port));
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    }

    pub async fn load_progress() -> Result<impl Reply, Infallible> {
        let msg = format!("{}", STATE.lock().unwrap().reading_done);
        Ok(warp::reply::with_status(msg, StatusCode::OK))
    }

    pub async fn toggle_lock() -> Result<impl Reply, Infallible> {
        Ok(warp::reply::with_status("", StatusCode::OK))
    }
}
