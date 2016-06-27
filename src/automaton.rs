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
pub enum Direction {
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum CellType {
    Water,
    Wall
}


impl Default for CellType {
    fn default() -> Self {
        CellType::Water
    }
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HPP {
    particles: [bool; 4],
    #[serde(skip_serializing)]
    stage: Stage,
    coord: (i32, i32),
    _type: CellType,
    #[serde(skip_serializing)]
    temp_particles: [bool; 4]
}


impl HPP {

    pub fn new(particles: [bool; 4], coord: (i32, i32), _type: CellType) -> Self {
        HPP {
            stage: Stage::Collision,
            particles: particles,
            coord: coord,
            _type: _type,
            temp_particles: particles
        }
    }
}


impl Cell for HPP {
    type Coord = (i32, i32);

    fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>> {

        if let (0, _) = self.coord {
            let mut new = self.clone();
            new.particles = [false, false, true, true];
            return new;
        }

        if let (119, _) = self.coord {
            let mut new = self.clone();
            new.particles = [false, false, false, false];
            return new;
        }

        match self._type {
            CellType::Wall => self.clone(),
            CellType::Water => match self.stage {
                Stage::Collision => self.collision(neighbors),
                Stage::Transport => self.transport(neighbors),
            }
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

    fn rebound(&self, d: &Direction) -> Option<(Direction, bool)> {
        if self.particle(&d) {
            let opposite = d.opposite();
            return Some((opposite, true));
        }
        else { return None; }
    }

    fn collision<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = HPP {
            stage: Stage::Transport,
            _type: self._type,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(neighbor) if neighbor._type == CellType::Wall => {

                    if let Some((dir, exists)) = self.rebound(&direction) {
                        new.set_particle(&dir, exists);
                    }
                },

                Some(neighbor) => {

                    let opposite = direction.opposite();
                    let head_on = self.particle(&direction) &&
                                  neighbor.particle(&opposite) &&
                                  !self.particle(&direction.perpendicular()) &&
                                  !neighbor.particle(&opposite.perpendicular());

                    if head_on {
                        new.set_particle(&direction.perpendicular(), true);
                    }
                    else {
                        let particle = new.particle(&direction) || self.particle(&direction);
                        new.set_particle(&direction, particle);
                    }
                },

                None => {

                    if let Some((dir, exists)) = self.rebound(&direction) {
                        new.set_particle(&dir, exists);
                    }
                }
            }
        }

        new
    }

    fn transport<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = HPP {
            stage: Stage::Collision,
            _type: self._type,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(neighbor) => {
                    let opposite = direction.opposite();

                    if neighbor.particle(&opposite) {
                        new.set_particle(&opposite, neighbor.particle(&opposite));
                    }
                },
                None => { 

                    if self.particle(&direction) {
                        new.set_particle(&direction.opposite(), true);
                    }
                }
            }
        }

        new
    }

    pub fn particle(&self, direction: &Direction) -> bool {

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
    pub fn directions(&self) -> [Direction; 4] {
        [Direction::Up, Direction::Left, Direction::Right, Direction::Down]
    }
}
