use std::path::PathBuf;

use crate::utils::browser::open_url;

pub async fn local_command(file: &PathBuf) {
    let port = portpicker::pick_unused_port().expect("No ports free");

    let r = api_routes::routes();

    open_url(&format!("http://localhost:{}", port));
    warp::serve(r).run(([127, 0, 0, 1], port)).await;
}

mod api_routes {
    use warp::{Filter, Rejection, Reply};

    use crate::commands::local::local_command::api_handlers;
    use crate::utils::webpage::host_webpage;

    pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        webpage().or(load_progress())
    }

    fn webpage() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        let index = warp::path::end().map(|| host_webpage("index.html"));
        let static_files = warp::path!(String).map(|path: String| host_webpage(&path));
        index.or(static_files)
    }

    fn load_progress() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        warp::path!("api" / "load_progress").and_then(api_handlers::load_progress)
    }
}

mod api_handlers {
    use std::convert::Infallible;

    use warp::Reply;

    pub async fn load_progress() -> Result<impl Reply, Infallible> {
        Ok(warp::reply::html("0"))
    }
}
