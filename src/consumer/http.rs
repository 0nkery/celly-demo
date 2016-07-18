use iron::prelude::*;
use iron::status;
use iron::mime::Mime;


pub fn http() {

    const INDEX_HTML: &'static [u8] = include_bytes!("index.html");

    fn index_handler(_: &mut Request) -> IronResult<Response> {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, INDEX_HTML)))
    }

    let chain = Chain::new(index_handler);

    println!("Index page is served on http:://localhost:8000/.");
    Iron::new(chain).http("127.0.0.1:8000").unwrap();
}