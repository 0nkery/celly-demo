mod http;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use celly::traits::Consumer;
use celly::traits::Grid;

use ws::{ Message, Sender, WebSocket };

use serde::Serialize;
use rmp_serde::Serializer;

use automaton::HPP;

use self::http::http;


pub struct IronWebConsumer {
    thandles: Vec<thread::JoinHandle<()>>,
    ws: Option<Sender>
}


impl IronWebConsumer {

    pub fn new() -> Self {

        let mut consumer = IronWebConsumer {
            thandles: Vec::new(),
            ws: None
        };
        consumer.start();

        consumer
    }

    fn start(&mut self) {

        self.thandles.push( thread::spawn(http) );

        let (tx, rx) = channel();

        let ws_handle = thread::spawn(move || {
            // Socket ignores all incoming messages.
            let sock = WebSocket::new(|_| { move |_| Ok(()) }).unwrap();
            // Passing sender back to main thread.
            tx.send(sock.broadcaster()).unwrap();

            println!("WS server is up and running on ws://localhost:3000/.");
            sock.listen("127.0.0.1:3000").unwrap();
        });
        self.thandles.push(ws_handle);

        let sender = rx.recv().unwrap();
        self.ws = Some(sender);
    }
}


impl Consumer for IronWebConsumer {

    type Cell = HPP;

    fn consume<G: Grid<Cell=Self::Cell>>(&mut self, grid: &mut G) {

        let mut buf = Vec::new();
        let res = grid.cells().serialize(&mut Serializer::new(&mut buf));

        match res {
            Ok(_) => {},
            Err(e) => {
                println!("MSGPACK|ERROR: {}", e);
                return;
            }
        };

        if let Some(ref ws) = self.ws {
            let res = ws.send(Message::binary(buf));

            if let Err(e) = res {
                println!("WS|ERROR: {}", e);
            }
        }

        thread::sleep(Duration::from_millis(200));
    }
}
