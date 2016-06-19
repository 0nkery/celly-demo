#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate celly;

extern crate iron;
extern crate logger;

extern crate ws;

extern crate serde;
extern crate rmp_serde;

extern crate rand;


use celly::grid::nhood::VonNeumannNhood;
use celly::grid::square::SquareGrid;
use celly::engine::Sequential;
use celly::traits::Engine;
use celly::traits::Grid;

use rand::Rng;

mod automaton;
mod consumer;

use automaton::HPP;
use automaton::CellType;
use consumer::IronWebConsumer;


fn main() {
    let nhood = VonNeumannNhood::new();
    let mut grid = SquareGrid::new(50, 120, nhood);

    let mut cells = Vec::new();

    let mut rnd = rand::thread_rng();
    for x in 0 .. 30 {
        for y in 0 .. 50 {
            cells.push(HPP::new(
                [rnd.gen(), rnd.gen(), rnd.gen(), rnd.gen()],
                (x, y),
                CellType::Water
            ));    
        }
        
    }

    for y in (0 .. 35).chain(40 .. 50) {
        cells.push(HPP::new(
            Default::default(),
            (30, y),
            CellType::Wall
        ));
    }

    for y in (0 .. 20).chain(25 .. 50) {
        cells.push(HPP::new(
            Default::default(),
            (90, y),
            CellType::Wall
        ));
    }

    grid.set_cells(cells);

    let consumer = IronWebConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(1000000);
}