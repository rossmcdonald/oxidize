use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;
use time::precise_time_ns;

pub struct Timing;

impl typemap::Key for Timing { type Value = u64; }

impl BeforeMiddleware for Timing {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        // Insert our start time into the request
        req.extensions.insert::<Timing>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for Timing {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<Timing>().unwrap();
        req.extensions.insert::<Timing>(delta);
        Ok(res)
    }
}
