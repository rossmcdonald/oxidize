use iron::AfterMiddleware;
use iron::prelude::*;
use middleware::timing::ResponseTime;
use time::now_utc;

pub struct LoggingMiddleware;

impl AfterMiddleware for LoggingMiddleware {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        // let uri: String = req.url.path.iter().fold("".to_string(), |s, item| s + "/" + item);
        // let uri: String = req.url.path;
        let timestamp = now_utc();
        let t: u64 = *req.extensions.get::<ResponseTime>().unwrap();
        info!("{} - {} - {} - {} - {}ms - {}",
              timestamp.rfc3339(),
              req.method,
              res.status.unwrap().to_u16(),
              req.remote_addr,
              (t as f64) / 1000000.0,
              req.url);
        Ok(res)
    }
}
