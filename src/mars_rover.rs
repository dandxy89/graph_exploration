//! ¢ Mars Rover Challenge
//!
//! ## Challenge:
//! The next Mars Rover is being developed, and we need you to come up with a
//! simple way of issuing navigation instructions to it from back on Earth!
//!
//! ### Part 1: Basic Movement
//! 1. The Mars Rover operates on a grid of arbitrary size.
//! 2. You can only issue three commands: Move forward, rotate clockwise, and rotate anticlockwise.
//! 3. If the rover moves off the grid, it reappears on the opposite side of the grid.
//!
//! ### Part 2: Autopilot
//! 1. Devise a simple process for determining the shortest possible path from
//!     one position on the grid to another.
//! 2. Improve the solution so that it can avoid mountain ranges that occupy a
//!     number of inconvenient grid squares scattered around the map.
//!
//! Tip of the hat to Mr Smith!
//!

use std::collections::{HashSet, VecDeque};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coord(usize, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    width: usize,
    height: usize,
    obstacles: HashSet<Coord>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            obstacles: HashSet::new(),
        }
    }

    // Add mountain ranges as per task description
    //
    pub fn add_terrain(self, obstacles: HashSet<Coord>) -> Self {
        Self { obstacles, ..self }
    }

    // For a given position get all surrounding non-obstacle `Coord`s
    //
    pub fn cord_neighbours(&self, position: &Coord) -> VecDeque<Coord> {
        VecDeque::from([
            self.step(&CardinalDir::North, position, 1),
            self.step(&CardinalDir::East, position, 1),
            self.step(&CardinalDir::South, position, 1),
            self.step(&CardinalDir::West, position, 1),
        ])
        .into_iter()
        .filter(|p| p != position)
        .collect()
    }

    // Let the Rover attempt to take N steps in a given direction.
    //
    // If an obstacle is encountered then stop at the furthest position
    //
    pub fn step(&self, dir: &CardinalDir, location: &Coord, steps: usize) -> Coord {
        let mut count = 0;
        let mut new_position: Coord = location.clone();

        // Walk one step at a time. If an obstacle is observed then stop.
        while count < steps {
            let position = match dir {
                CardinalDir::East | CardinalDir::West => {
                    let x = (location.0 as isize + dir.v()).rem_euclid(self.width as isize);
                    Coord(x as usize, location.1)
                }
                CardinalDir::North | CardinalDir::South => {
                    let y = (location.1 as isize + dir.v()).rem_euclid(self.height as isize);
                    Coord(location.0, y as usize)
                }
            };

            if self.obstacles.contains(&position) {
                return new_position;
            }

            new_position = position;
            count += 1;
        }

        new_position
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MarsRover {
    position: Coord,
    direction: CardinalDir,
}

// Helper function to clean up the history
//
fn get_history(history: Vec<Coord>, g: &Grid) -> Vec<Coord> {
    let mut pathway = VecDeque::new();
    for c in history.into_iter().rev() {
        if let Some(nxt) = pathway.front() {
            if g.cord_neighbours(&c).iter().any(|p| p == nxt) {
                pathway.push_front(c);
            }
        } else {
            pathway.push_front(c);
        }
    }

    pathway.into_iter().collect()
}

impl MarsRover {
    pub fn new(position: Coord, direction: CardinalDir) -> Self {
        Self {
            position,
            direction,
        }
    }

    // Find a path to a given target node
    //
    // if you skint this is essentially the DFS algorithm
    //
    pub fn auto_pilot(&self, g: &Grid, target: &Coord) -> Option<Vec<Coord>> {
        let mut visited = HashSet::new();
        let mut history = Vec::new();
        let mut queue = VecDeque::new();

        visited.insert(self.position.clone());
        queue.push_back(self.position.clone());

        while let Some(current) = queue.pop_front() {
            history.push(current.clone());

            if current == *target {
                return Some(get_history(history, g));
            }

            for n in g.cord_neighbours(&current) {
                if !visited.contains(&n) {
                    visited.insert(n.clone());
                    queue.push_back(n);
                }
            }
        }

        None
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
                position: g.step(&self.direction, &self.position, *steps),
                ..self
            },
            Command::Clockwise => self.rotate_clockwise(),
            Command::AntiClockwise => self.rotate_anticlockwise(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_turning_actions() {
        let grid = Grid::new(20, 20);
        let rover = MarsRover::new(Coord(0, 0), CardinalDir::East);

        let command_one = Command::Clockwise;
        let rover = rover.execute_command(&command_one, &grid);
        assert!(rover.direction == CardinalDir::South);

        let command_two = Command::AntiClockwise;
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

    #[test]
    fn test_moving_into_an_obstacle() {
        let obstacles = HashSet::from([Coord(0, 1), Coord(1, 0)]);
        let grid = Grid::new(20, 20).add_terrain(obstacles);
        let rover = MarsRover::new(Coord(0, 0), CardinalDir::East);

        let command_one = Command::Forward(1);
        let rover = rover.execute_command(&command_one, &grid);
        assert_eq!(0, rover.position.0);
        assert_eq!(0, rover.position.1);
    }

    #[test]
    fn test_auto_pilot() {
        let obstacles = HashSet::from([
            Coord(0, 1),
            Coord(1, 1),
            Coord(2, 1),
            Coord(3, 1),
            Coord(4, 1),
        ]);
        let grid = Grid::new(5, 3).add_terrain(obstacles);
        let rover = MarsRover::new(Coord(0, 0), CardinalDir::East);

        let route = rover.auto_pilot(&grid, &Coord(2, 2));
        assert_eq!(4, route.unwrap().len());
    }
}
