# oxidize

A simple web application built in Rust leveraging Iron and Hyper.

## Running

This was developed using `rustc 1.4.0-nightly (e5d90d984 2015-08-07)`. To run:

```
$ export RUST_LOG=oxidize
$ cargo run
```

By default, the application will bind to `localhost:3000`. As of right now, there is only one endpoint: `/api`. Running `curl`, you should see something similar to:

```
$ curl -v 'http://127.0.0.1:3000/api/test'
* Hostname was NOT found in DNS cache
*   Trying 127.0.0.1...
* Connected to 127.0.0.1 (127.0.0.1) port 3000 (#0)
> GET /api/test HTTP/1.1
> User-Agent: curl/7.38.0
> Host: 127.0.0.1:3000
> Accept: */*
>
< HTTP/1.1 200 OK
< Content-Type: text/plain
< Content-Length: 25
< Date: Fri, 04 Sep 2015 17:54:09 GMT
<
Hello!

* Connection #0 to host 127.0.0.1 left intact
Object ID: 'test'
```

All other endpoints will most likely generate errors.