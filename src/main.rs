#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate celly;

extern crate iron;
extern crate logger;

extern crate websocket;

extern crate serde;


use celly::grid::nhood::VonNeumannNhood;
use celly::grid::square::SquareGrid;
use celly::engine::Sequential;
use celly::traits::Engine;

mod automaton;
mod consumer;

use automaton::HPP;
use consumer::IronWebConsumer;


fn main() {
    let nhood = VonNeumannNhood::new();
    let grid: SquareGrid<HPP, _> = SquareGrid::new(100, 100, nhood);

    let consumer = IronWebConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(100000);
}