use std::thread;

use celly::traits::Grid;
use celly::traits::ReprConsumer;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use logger::Logger;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use websocket::message::Type;


pub struct IronWebConsumer {}


impl IronWebConsumer {

    pub fn new() -> Self {

        let consumer = IronWebConsumer { };
        consumer.start();

        consumer
    }

    fn start(&self) {

        thread::spawn(http);

        thread::spawn(move || {
            let ws = WebsocketServer::new();
            ws.listen();
        });
    }
}


impl ReprConsumer for IronWebConsumer {

    fn consume<G: Grid>(&mut self, grid: &G) {
        // send to ws server
    }
}


fn http() {

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


struct WebsocketServer<'a> {
    srv: Server<'a>,
}


impl<'a> WebsocketServer<'a> {

    pub fn new() -> Self {


        let ws_server = WebsocketServer {
            srv: Server::bind("127.0.0.1:3000").unwrap(),
        };

        ws_server
    }

    pub fn listen(self) {
        println!("Websocket server is up and listening on 127.0.0.1:3000.");

        for conn in self.srv {

            let request = conn.unwrap().read_request().unwrap();
            let headers = request.headers.clone();

            request.validate().unwrap();
            let mut response = request.accept();

            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                if protocols.contains(&("rust-websocket".to_string())) {
                    response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
                }
            }

            let mut client = response.send().unwrap();

            let ip = client.get_mut_sender()
                           .get_mut()
                           .peer_addr()
                           .unwrap();

            println!("Connection from {}.", ip);

            let (mut sender, mut receiver) = client.split();

            // receive
            thread::spawn(move || {
                for message in receiver.incoming_messages() {
                    let message: Message = message.unwrap();

                    match message.opcode {

                        Type::Close => {
                            let message = Message::close();
                            sender.send_message(&message).unwrap();
                            println!("Client {} disconnected.", ip);
                            return;
                        },

                        Type::Ping => {
                            let message = Message::pong(message.payload);
                            sender.send_message(&message).unwrap();
                        },

                        _ => { },
                    }
                }
            });

            // send
            thread::spawn(move || {

            });
        }
    }
}
