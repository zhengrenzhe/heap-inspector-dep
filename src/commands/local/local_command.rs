use std::cell::RefCell;
use std::convert::Infallible;
use std::path::PathBuf;

use warp::http::StatusCode;
use warp::{Filter, Reply};

use crate::commands::local::thread::AnalyseThread;
use crate::utils::browser::open_url;
use crate::utils::webpage::webpage_routes;

thread_local!(static FOO: RefCell<AnalyseThread> = RefCell::new(
    AnalyseThread {
        lock: false
    }
));

pub struct LC {
    pub(crate) file_path: PathBuf,
}

impl LC {
    pub async fn start(&self) {
        let port = portpicker::pick_unused_port().expect("No ports free");

        let routes = webpage_routes()
            .or(warp::path!("api" / "lock_status").and_then(|| LC::lock_status()))
            .or(warp::path!("api" / "toggle_lock").and_then(LC::toggle_lock));

        open_url(&format!("http://localhost:{}", port));
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    }

    pub async fn lock_status() -> Result<impl Reply, Infallible> {
        FOO.with(|f| {
            println!("is lock {:?}", f.borrow().lock);
        });
        Ok(warp::reply::with_status("", StatusCode::OK))
    }

    pub async fn toggle_lock() -> Result<impl Reply, Infallible> {
        FOO.with(|f| {
            let s = { f.borrow().lock };
            f.borrow_mut().lock = !s;
        });
        Ok(warp::reply::with_status("", StatusCode::OK))
    }
}
