extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate time;

use iron::prelude::*;
use iron::mime::Mime;
use iron::method::Method::*;
use iron::headers::*;
use router::{Router};
use iron::status;

use persistent::Read;

mod middleware {
    pub mod timing;
    pub mod logging;
}

use middleware::timing::ResponseTime;
use middleware::logging::LoggingMiddleware;

// Arbitrary limit on JSON body length
const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn api(req: &mut Request) -> IronResult<Response> {
    match req.method {
        Get => {
            // Collect object ID (or set it to empty string if not found)
            let obj_id = req.extensions.get::<Router>().unwrap().find("obj_id").unwrap_or("");
            
            let user_agent = req.headers.get::<UserAgent>().unwrap();
            // TODO Perform some simple vetting of user-agent to discern browser v programmatic/API access
            let content_type = "text/plain".parse::<Mime>().unwrap();
            
            let response_body: String = format!("Hello!\n\nObject ID: \'{}\'", obj_id);
            Ok(Response::with((content_type, status::Ok, response_body)))
        },
        Put => {
            // TODO Not implemented yet
            Ok(Response::with((status::Ok)))
        },
        Post => {
            let json_data = req.get::<bodyparser::Json>();
            match json_data {
                Ok(Some(data)) => {
                    info!("Data received!");
                    // TODO Use/store JSON data
                    // for (key, value) in data.as_object().unwrap().iter() {
                    //     println!("\t- {}: {}", key, value);
                    // }
                },
                Ok(None) => info!("No data received!"),
                Err(e) => {
                    error!("{}", e);
                    return Ok(Response::with((status::BadRequest)))
                }
            }
            Ok(Response::with((status::Ok)))
        },
        Delete => {
            //TODO Not implemented yet
            Ok(Response::with((status::Ok)))
        },
        _ => {
            warn!("Unrecognized request method.");
            Ok(Response::with((status::BadRequest)))
        }
    }
}

fn generate_router() -> Router {
    info!("Generating router...");
    let mut r = Router::new();
    
    r.get("/api", api);
    r.get("/api/:obj_id", api);
    r.post("/api", api);
    r.put("/api/:obj_id", api);
    r.delete("/api/:obj_id", api);    
    r
}

fn generate_chain() -> Chain {
    info!("Mapping middleware...");
    let mut c = Chain::new(generate_router());
    c.link_before(ResponseTime);
    c.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    c.link_after(ResponseTime);
    c.link_after(LoggingMiddleware);
    c
}

fn main() {
    // Initialize logging (level is controlled by $RUST_LOG env variable, ex: RUST_LOG=oxidize)
    env_logger::init().unwrap();

    let chain = generate_chain();
    let binding = "localhost:3000";
    info!("Starting server with binding: {}", binding);
    Iron::new(chain).http(binding).unwrap();
}
