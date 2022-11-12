//! Mars Rover Challenge
//!
//! Challenge:
//! The next Mars Rover is being developed, and we need you to come up with a
//! simple way of issuing navigation instructions to it from back on Earth!
//!
//! Part 1: Basic Movement
//! 1. The Mars Rover operates on a grid of arbitrary size.
//! 2. You can only issue three commands: Move forward, rotate clockwise, and rotate anticlockwise.
//! 3. If the rover moves off the grid, it reappears on the opposite side of the grid.
//! --> IMPLEMENTED
//!
//! Part 2: Autopilot
//! 1. Devise a simple process for determining the shortest possible path from
//!     one position on the grid to another.
//! 2. Improve the solution so that it can avoid mountain ranges that occupy a
//!     number of inconvenient grid squares scattered around the map.
//!
//! Part 3: Putting it all together
//! 1. Output all the instructions and moves carried out by the rover to get
//!     from one grid square to another.
//!

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Command {
    Forward(usize),
    Clockwise,
    AntiClockwise,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CardinalDir {
    North,
    East,
    South,
    West,
}

impl CardinalDir {
    pub fn v(&self) -> isize {
        match self {
            Self::North | Self::East => 1,
            Self::South | Self::West => -1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coord(usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Grid {
    width: usize,
    height: usize,
    obstacles: Vec<Coord>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            obstacles: Vec::with_capacity(0),
        }
    }
    
    // Add mountain ranges...
    pub fn add_terrain(self, obstacles: Vec<Coord>) -> Self {
        Self { obstacles, ..self }
    }

    pub fn move_forward(&self, dir: &CardinalDir, location: &Coord, steps: usize) -> Coord {
        match dir {
            CardinalDir::East | CardinalDir::West => {
                let x = (location.0 as isize + steps as isize * dir.v())
                    .rem_euclid(self.width as isize);

                Coord(x as usize, location.1)
            }
            CardinalDir::North | CardinalDir::South => {
                let y = (location.1 as isize + steps as isize * dir.v())
                    .rem_euclid(self.height as isize);

                Coord(location.0, y as usize)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MarsRover {
    position: Coord,
    direction: CardinalDir,
}

impl MarsRover {
    pub fn new(position: Coord, direction: CardinalDir) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn rotate_clockwise(self) -> Self {
        Self {
            direction: match self.direction {
                CardinalDir::North => CardinalDir::East,
                CardinalDir::East => CardinalDir::South,
                CardinalDir::South => CardinalDir::West,
                CardinalDir::West => CardinalDir::North,
            },
            ..self
        }
    }

    fn rotate_anticlockwise(self) -> Self {
        Self {
            direction: match self.direction {
                CardinalDir::North => CardinalDir::West,
                CardinalDir::East => CardinalDir::North,
                CardinalDir::South => CardinalDir::East,
                CardinalDir::West => CardinalDir::South,
            },
            ..self
        }
    }

    pub fn execute_command(self, command: &Command, g: &Grid) -> Self {
        match command {
            Command::Forward(steps) => Self {
                position: g.move_forward(&self.direction, &self.position, *steps),
                ..self
            },
            Command::Clockwise => self.rotate_clockwise(),
            Command::AntiClockwise => self.rotate_anticlockwise(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turning_actions() {
        let grid = Grid::new(20, 20);
        let rover = MarsRover::new(Coord(0, 0), CardinalDir::East);

        let command_one = Command::Clockwise;
        let rover = rover.execute_command(&command_one, &grid);
        assert!(rover.direction == CardinalDir::South);

        let command_two = Command::Clockwise;
        let rover = rover.execute_command(&command_two, &grid);
        assert!(rover.direction == CardinalDir::East);
    }

    #[test]
    fn test_moving() {
        let grid = Grid::new(20, 20);
        let rover = MarsRover::new(Coord(0, 0), CardinalDir::East);

        let command_one = Command::Forward(1);
        let rover = rover.execute_command(&command_one, &grid);
        assert_eq!(1, rover.position.0);
        assert_eq!(0, rover.position.1);
    }

    #[test]
    fn test_moving_off_end() {
        let grid = Grid::new(20, 20);
        let rover = MarsRover::new(Coord(0, 0), CardinalDir::West);

        let command_one = Command::Forward(1);
        let rover = rover.execute_command(&command_one, &grid);
        assert_eq!(19, rover.position.0);
        assert_eq!(0, rover.position.1);
    }
}
