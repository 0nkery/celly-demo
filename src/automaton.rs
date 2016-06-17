use celly::traits::Cell;
use celly::traits::Coord;

/// Implementation of [HPP model](https://en.wikipedia.org/wiki/HPP_model).
/// Assumes Von Nuemann's neighborhood.

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum Stage {
    Collision,
    Transport
}


impl Default for Stage {
    fn default() -> Self { Stage::Collision }
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}


impl Direction {

    fn opposite(&self) -> Self {

        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up
        }
    }

    fn perpendicular(&self) -> Self {

        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right
        }
    }
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HPP {
    particles: [bool; 4],
    stage: Stage,
    coord: (i32, i32)
}


impl Cell for HPP {
    type Coord = (i32, i32);

    fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>> {

        match self.stage {
            Stage::Collision => self.collision(neighbors),
            Stage::Transport => self.transport(neighbors),
        }
    }

    fn with_coord<C: Coord>(coord: C) -> Self {
        HPP { 
            stage: Stage::Collision,
            coord: (coord.x(), coord.y()),
            ..Default::default()
        }
    }

    fn coord(&self) -> &Self::Coord {
        &self.coord
    }

    fn set_coord<C: Coord>(&mut self, coord: &C) {
        self.coord = (coord.x(), coord.y());
    }
}


impl HPP {

    fn collision<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = HPP {
            stage: Stage::Transport,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(neighbor) => {
                    let opposite = direction.opposite();
                    let head_on = self.particle(&direction) && neighbor.particle(&opposite);

                    if head_on {
                        new.set_particle(&direction.perpendicular(), self.particle(&direction));
                    }
                    else {
                        let particle = new.particle(&direction) || self.particle(&direction);
                        new.set_particle(&direction, particle);
                    }
                },
                // Rebound
                None => {
                    let opposite = direction.opposite();
                    new.set_particle(&opposite, self.particle(direction));
                }
            }
        }

        new
    }

    fn transport<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = HPP {
            stage: Stage::Collision,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {
                Some(neighbor) => {
                    let opposite = direction.opposite();
                    new.set_particle(
                        &opposite,
                        neighbor.particle(&opposite) || self.particle(&opposite)
                    );
                },
                None => {
                    new.set_particle(&direction, self.particle(&direction))
                }
            }
        }

        new
    }

    fn particle(&self, direction: &Direction) -> bool {

        match *direction {
            Direction::Up => self.particles[0],
            Direction::Left => self.particles[1],
            Direction::Right => self.particles[2],
            Direction::Down => self.particles[3]
        }
    }

    fn set_particle(&mut self, direction: &Direction, exists: bool) {

        match *direction {
            Direction::Up => { self.particles[0] = exists },
            Direction::Left => { self.particles[1] = exists },
            Direction::Right => { self.particles[2] = exists },
            Direction::Down => { self.particles[3] = exists }
        }
    }

    #[inline]
    fn directions(&self) -> [Direction; 4] {
        [Direction::Up, Direction::Left, Direction::Right, Direction::Down]
    }
}
