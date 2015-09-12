#![feature(plugin)]
#![plugin(maud_macros)]
#[macro_use]
extern crate maud;

extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate time;

use iron::prelude::*;
use router::{Router};

use persistent::Read;

mod middleware;
mod handlers;

// Arbitrary limit on JSON body length
const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn generate_router() -> Router {
    info!("Generating router...");
    let mut r = Router::new();
    
    r.get("/api", handlers::api);
    r.get("/api/:obj_id", handlers::api);
    r.post("/api", handlers::api);
    r.put("/api/:obj_id", handlers::api);
    r.delete("/api/:obj_id", handlers::api);
    r
}

fn generate_chain() -> Chain {
    info!("Mapping middleware...");
    let mut c = Chain::new(generate_router());
    c.link_before(middleware::Timing);
    c.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    c.link_after(middleware::Timing);
    c.link_after(middleware::Logging);
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
