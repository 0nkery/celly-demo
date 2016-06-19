use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use logger::Logger;


pub fn http() {

    const INDEX_HTML: &'static [u8] = include_bytes!("index.html");

    fn index_handler(_: &mut Request) -> IronResult<Response> {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, INDEX_HTML)))
    }

    let mut chain = Chain::new(index_handler);

    let (logger_before, logger_after) = Logger::new(None);

    chain.link_before(logger_before);
    chain.link_after(logger_after);

    println!("Index page is served on http:://localhost:8000/.");
    Iron::new(chain).http("127.0.0.1:8000").unwrap();
}