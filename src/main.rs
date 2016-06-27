#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate celly;

extern crate iron;
extern crate logger;

extern crate ws;

extern crate serde;
extern crate rmp_serde;

use celly::grid::nhood::VonNeumannNhood;
use celly::grid::square::SquareGrid;
use celly::engine::Sequential;
use celly::traits::Engine;
use celly::traits::Grid;

mod automaton;
mod consumer;

use automaton::HPP;
use automaton::CellType;
use consumer::IronWebConsumer;


fn main() {
    let nhood = VonNeumannNhood::new();
    let mut grid = SquareGrid::new(30, 70, nhood);

    let mut cells = Vec::new();

    let mut x = 30;
    let mut y = 10;
    let mut x_sign = 1;
    let mut y_sign = 1;

    while x != 30 || y != 10 || y_sign > 0 {
        cells.push(HPP::new(
            [false, false, false, false],
            (x, y),
            CellType::Wall
        ));

        x += x_sign;
        y += y_sign;

        if x == 35 {
            x_sign = -x_sign;
        }

        if x == 25 {
            x_sign = -x_sign;
        }

        if y == 20 {
            y_sign = -y_sign;
        }

        println!("({}, {})", x, y);
    }

    grid.set_cells(cells);

    let consumer = IronWebConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(1000000);
}