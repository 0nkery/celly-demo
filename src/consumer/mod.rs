mod http;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use celly::traits::Consumer;
use celly::traits::Coord;
use celly::traits::Grid;

use ws::{ Message, Sender, WebSocket };

use serde::Serialize;
use rmp_serde::Serializer;

use automaton::HPP;
use automaton::CellType;

use self::http::http;


pub struct IronWebConsumer {
    thandles: Vec<thread::JoinHandle<()>>,
    ws: Option<Sender>,
    springhead: Option<Vec<HPP>>,
    estuary: Option<Vec<HPP>>,
}


impl IronWebConsumer {

    pub fn new() -> Self {

        let mut consumer = IronWebConsumer {
            thandles: Vec::new(),
            ws: None,
            springhead: None,
            estuary: None
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

    fn prepare_grid_layout(&mut self, rows: i32, cols: i32) {
        if self.springhead.is_none() {
            let mut springhead_vec = Vec::with_capacity(rows as usize);
            let mut estuary_vec = Vec::with_capacity(rows as usize);

            let springhead_ps = [true, true, false, true];
            let estuary_ps = [false, false, false, false];

            let last_x = cols - 1;
            let cell_t = CellType::Water;

            for y in 0 .. rows {
                springhead_vec.push(HPP::new(springhead_ps, (0, y), cell_t));
                estuary_vec.push(HPP::new(estuary_ps, (last_x, y), cell_t));
            }

            self.springhead = Some(springhead_vec);
            self.estuary = Some(estuary_vec);
        }
    }
}


impl Consumer for IronWebConsumer {

    type Cell = HPP;

    fn consume<G: Grid<Cell=Self::Cell>>(&mut self, grid: &mut G) {

        let grid_ds = grid.dimensions();
        self.prepare_grid_layout(grid_ds.y(), grid_ds.x());

        let springhead = self.springhead.as_ref().unwrap().clone();
        let estuary = self.estuary.as_ref().unwrap().clone();
        grid.set_cells(springhead);
        grid.set_cells(estuary);

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

        thread::sleep(Duration::from_millis(400));
    }
}
