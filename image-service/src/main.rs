#[cfg(all(test, unix))]
extern crate nix;

extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use futures::prelude::*;
use gotham::helpers::http::response::create_empty_response;
use hyper::{Body, Response, StatusCode};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use tokio::signal;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct PathExtractor {
    id: usize,
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/images/:id:[0-9]+")
            .with_path_extractor::<PathExtractor>()
            .to(get_image);
      //  route.associate("/images", |assoc| {
      //      assoc.get().to();
      //      assoc.post().to();
      //      assoc.patch().to();
       // })
    })
}

#[tokio::main]
pub async fn main() {
    let addr = "0.0.0.0:7878";
    println!("Listening for request at http://{}", addr);
    let server = gotham::init_server(addr, router());

    let signal = async {
        signal::ctrl_c().map_err(|_| ()).await?;
        Ok::<(), ()>(())
    };

    future::select(Box::new(server), Box::new(signal)).await;

}

fn get_image(state: State) -> (State, Response<Body>) {
    //let path = PathExtractor::borrow_from(&state);
    let res = create_empty_response(&state, StatusCode::OK);
    
    (state, res)
}

#[cfg(test)]
mod tests {

    use super::*;
    use gotham::test::TestServer;
    #[cfg(unix)]
    use hyper::Client;
    #[cfg(unix)]
    use nix::sys::signal::{kill, Signal};
    use std::thread;
    use std::time::Duration;
    #[cfg(unix)]
    use tokio::runtime::Runtime;
    
    #[test]
    fn receive_success_image_get_id() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server.client()
            .get("http://localhost/images/123")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn receive_missing_image_get_id() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server.client()
            .get("http://localhost/images/abv")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[cfg(unix)]
    fn try_request() -> bool {
        let client = Client::new();
        let uri = "http://localhost/images";
        let uri_parsed = uri.parse().unwrap();
        let work = client.get(uri_parsed);

        let mut rt = Runtime::new().unwrap();

        match rt.block_on(work) {
            Ok(req) => {
                assert_eq(req.status, StatusCode::OK);
            }

            Err(error) => {
                eprintln!("Unable to get \"{}\": {}", uri, error);
                false
            }
        }

    }

    #[cfg(unix)]
    #[test]
    fn test_gracefull_shutdown() {
        let thread_handle = thread::spawn(main);

        //Wait until server will be able to answer
        let mut max_retries = 25;
        while (max_retries != 0) && !try_request() {
            max_retries -= 1;
            thread::sleep(Duration::from_millis(200));
        }
        assert_ne!(max_retries, 0);

        //Send SIGINT to self
        kill(nix::unistd::getpid(), Signal::SIGINT).unwrap();
        thread_handle.join().unwrap();
    }

}
