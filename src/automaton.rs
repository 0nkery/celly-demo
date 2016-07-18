use celly::traits::Cell;
use celly::traits::Coord;

/// Implementation of [HPP model](https://en.wikipedia.org/wiki/HPP_model).
/// Assumes Von Nuemann's neighborhood.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum Stage {
    Collision,
    Transport,
}


impl Default for Stage {
    fn default() -> Self {
        Stage::Collision
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Down,
    Right,
    Left,
    Up,
}


impl Direction {
    fn opposite(&self) -> Self {

        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    fn perpendicular(&self) -> Self {

        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum CellType {
    Water,
    Wall,
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
}


impl HPP {
    pub fn new(particles: [bool; 4], coord: (i32, i32), _type: CellType) -> Self {
        HPP {
            stage: Stage::Collision,
            particles: particles,
            coord: coord,
            _type: _type,
        }
    }
}


impl Cell for HPP {
    type Coord = (i32, i32);

    fn step<'a, I>(&'a self, neighbors: I) -> Self
        where I: Iterator<Item = Option<&'a Self>>
    {

        match self._type {
            CellType::Wall => self.clone(),
            CellType::Water => {
                match self.stage {
                    Stage::Collision => self.collision(neighbors),
                    Stage::Transport => self.transport(neighbors),
                }
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
    fn collision<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item = Option<&'a Self>>
    {

        let mut new = HPP {
            stage: Stage::Transport,
            _type: self._type,
            ..Default::default()
        };

        let has_head_on = |d: &Direction, op_d: &Direction| {
            self.particle(&d) && self.particle(&op_d) && !self.particle(&d.perpendicular()) &&
            !self.particle(&op_d.perpendicular())
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(neighbor) if neighbor._type == CellType::Wall => {
                    let opposite = direction.opposite();
                    let head_on = has_head_on(&direction, &opposite);
                    if head_on {
                        new.set_particle(&direction.perpendicular(), true);
                        new.set_particle(&opposite.perpendicular(), true);
                    } else {
                        if self.particle(&direction) {
                            new.set_particle(&opposite, true);
                        }
                    }
                }

                Some(_) => {

                    let opposite = direction.opposite();

                    let head_on = has_head_on(&direction, &opposite);
                    if head_on {
                        new.set_particle(&direction.perpendicular(), true);
                        new.set_particle(&opposite.perpendicular(), true);
                    } else {
                        let exists = new.particle(&direction) || self.particle(&direction);
                        new.set_particle(&direction, exists);
                    }
                }

                None => {
                    let opposite = direction.opposite();
                    let head_on = has_head_on(&direction, &opposite);
                    if head_on {
                        new.set_particle(&direction.perpendicular(), true);
                        new.set_particle(&opposite.perpendicular(), true);
                    } else {
                        if self.particle(&direction) {
                            new.set_particle(&opposite, true);
                        }
                    }
                }
            }
        }

        new
    }

    fn transport<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item = Option<&'a Self>>
    {

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
                        new.set_particle(&opposite, true);
                    }
                }
                None => {
                    if self.particle(&direction) {
                        new.set_particle(&direction, true);
                    }
                }
            }
        }

        new
    }

    pub fn particle(&self, direction: &Direction) -> bool {
        let index = *direction as usize;
        self.particles[index]
    }

    fn set_particle(&mut self, direction: &Direction, exists: bool) {
        let index = *direction as usize;
        self.particles[index] = exists;
    }

    #[inline]
    pub fn directions(&self) -> [Direction; 4] {
        [Direction::Up, Direction::Left, Direction::Right, Direction::Down]
    }
}
