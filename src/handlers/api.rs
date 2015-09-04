use bodyparser;

use iron::prelude::*;
use iron::mime::Mime;
use iron::method::Method::*;
use iron::headers::*;
use iron::status;

use router::{Router};

pub fn api(req: &mut Request) -> IronResult<Response> {
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
