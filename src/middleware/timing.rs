use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;
use time::precise_time_ns;

pub struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        // Insert our start time into the request
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        req.extensions.insert::<ResponseTime>(delta);
        Ok(res)
    }
}
