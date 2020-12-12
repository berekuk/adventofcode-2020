use std::io::{self, BufRead};

#[derive(Debug)]
enum LR {
    Left,
    Right,
}

impl LR {
    fn sign(&self) -> i32 {
        match self {
            LR::Left => -1,
            LR::Right => 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_degree(degree: i32) -> Direction {
        match (degree + 360) % 360 {
            0 => Direction::East,
            90 => Direction::South,
            180 => Direction::West,
            270 => Direction::North,
            _ => panic!("degree should be divisible by 90, got: {}", degree % 360),
        }
    }

    fn to_degree(&self) -> i32 {
        match self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => 270,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    fn turn(&self, lr: &LR, degree: i32) -> Direction {
        Direction::from_degree(self.to_degree() + lr.sign() * degree)
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_in_direction(&self, direction: &Direction, distance: i32) -> Position {
        let delta = direction.delta();
        Position {
            x: self.x + delta.0 * distance,
            y: self.y + delta.1 * distance,
        }
    }

    fn turn_around_zero(&self, lr: &LR, degree: i32) -> Position {
        match (degree * lr.sign() + 360) % 360 {
            0 => self.clone(),
            90 => Position { x: self.y, y: -self.x },
            180 => Position { x: -self.x, y: -self.y },
            270 => Position { x: -self.y, y: self.x },
            _ => panic!("bad degree"),
        }
    }
    
    fn manhattan_distance_from_zero(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone)]
struct Ship {
    position: Position,
    direction: Direction,
}

impl Ship {
    fn initial() -> Ship {
        Ship {
            position: Position { x: 0, y: 0 },
            direction: Direction::East,
        }
    }
}

#[derive(Debug, Clone)]
struct P2State {
    ship: Ship,
    waypoint: Position,
}

#[derive(Debug)]
enum Action {
    Turn(LR),
    Move(Direction),
    Forward,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32,
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let (action_str, value_str) = line.split_at(1);
        let action = match action_str {
            "L" => Action::Turn(LR::Left),
            "R" => Action::Turn(LR::Right),
            "N" => Action::Move(Direction::North),
            "S" => Action::Move(Direction::South),
            "E" => Action::Move(Direction::East),
            "W" => Action::Move(Direction::West),
            "F" => Action::Forward,
            _ => panic!("unknown action"),
        };
        let value = value_str.parse::<i32>().unwrap();
        Instruction { action, value }
    }

    fn apply_p1(&self, ship: &Ship) -> Ship {
        match &self.action {
            Action::Turn(lr) => {
                Ship {
                    position: ship.position.clone(),
                    direction: ship.direction.turn(&lr, self.value),
                }
            },
            Action::Move(direction) => {
                Ship {
                    position: ship.position.move_in_direction(&direction, self.value),
                    direction: ship.direction.clone(),
                }
            },
            Action::Forward => {
                Ship {
                    position: ship.position.move_in_direction(&ship.direction, self.value),
                    direction: ship.direction.clone(),
                }
            }
        }
    }

    fn apply_p2(&self, state: &P2State) -> P2State {
        match &self.action {
            Action::Turn(lr) => {
                P2State {
                    ship: state.ship.clone(),
                    waypoint: state.waypoint.turn_around_zero(&lr, self.value),
                }
            },
            Action::Move(direction) => {
                P2State {
                    ship: state.ship.clone(),
                    waypoint: state.waypoint.move_in_direction(&direction, self.value),
                }
            },
            Action::Forward => {
                P2State {
                    ship: Ship {
                        position: Position {
                            x: state.ship.position.x + state.waypoint.x * self.value,
                            y: state.ship.position.y + state.waypoint.y * self.value,
                        },
                        direction: state.ship.direction.clone(),
                    },
                    waypoint: state.waypoint.clone(),
                }
            },
        }
    }
}

struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    fn parse(input: &mut dyn BufRead) -> Instructions {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let line = line.unwrap();
            let line = line.trim();
            instructions.push(Instruction::parse(line));
        }
        Instructions { instructions }
    }

    fn apply_p1(&self, initial_ship: &Ship) -> Ship {
        let mut ship = initial_ship.clone();

        for instruction in self.instructions.iter() {
            println!("{:?}, {:?}", ship, instruction);
            ship = instruction.apply_p1(&ship);
        }

        ship
    }

    fn apply_p2(&self, initial_state: &P2State) -> P2State {
        let mut state = initial_state.clone();

        for instruction in self.instructions.iter() {
            println!("{:?}, {:?}", initial_state, instruction);
            state = instruction.apply_p2(&state);
        }

        state
    }
}

fn main() {
    let stdin = io::stdin();

    let instructions = Instructions::parse(&mut stdin.lock());
    let p1_ship = instructions.apply_p1(&Ship::initial());

    println!("{:?}", p1_ship);
    println!("{}", p1_ship.position.manhattan_distance_from_zero());

    let p2_state = instructions.apply_p2(&P2State {
        ship: Ship::initial(),
        waypoint: Position { x: 10, y: 1 },
    });

    println!("{:?}", p2_state);
    println!("{}", p2_state.ship.position.manhattan_distance_from_zero());
}
