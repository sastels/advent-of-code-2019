#[path = "intcode.rs"]
mod intcode;

use std::collections::HashSet;

#[derive(PartialEq, Clone, Copy)]
enum Color {
    Black,
    White,
}

#[derive(Clone)]
struct Robot {
    x: i32,
    y: i32,
    direction_angle: i32, // 0 up, 90 right, etc
    program: intcode::Program,
}

struct Hull {
    plates: Vec<Color>,
    width: usize,
    offset: i32,
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if *self == Color::White {
            write!(f, "1")
        } else {
            write!(f, "0")
        }
    }
}

impl Hull {
    fn new(width: usize) -> Hull {
        Hull {
            plates: vec![Color::Black; width * width],
            width: width,
            offset: width as i32 / 2,
        }
    }

    fn index_for(&self, x: i32, y: i32) -> usize {
        let offset_x = (self.offset + x) as usize;
        let offset_y = (self.offset + y) as usize;
        self.width * offset_y + offset_x
    }

    fn paint(&mut self, x: i32, y: i32, color_code: i32) {
        let index = self.index_for(x, y);
        if color_code == 0 {
            self.plates[index] = Color::Black;
        } else if color_code == 1 {
            self.plates[index] = Color::White;
        } else {
            panic!("Unknown color code {}", color_code)
        }
    }

    fn color_of_plate(&self, x: i32, y: i32) -> i32 {
        let color = self.plates[self.index_for(x, y)];
        if color == Color::Black {
            0
        } else {
            1
        }
    }
    fn print(&self) {
        for (i, color) in self.plates.iter().enumerate() {
            if i % self.width == 0 {
                println!(" ");
            }
            if *color == Color::Black {
                print!(" ");
            } else {
                print!("X");
            }
        }
    }
}

impl Robot {
    fn new(code: &str) -> Robot {
        Robot {
            x: 0,
            y: 0,
            direction_angle: 0,
            program: intcode::Program::new(code, ""),
        }
    }

    fn rotate(&mut self, direction_code: u32) {
        if direction_code == 0 {
            self.direction_angle = (self.direction_angle - 90) % 360;
        } else if direction_code == 1 {
            self.direction_angle = (self.direction_angle + 90) % 360;
        } else {
            panic!("Bad direction code {}", direction_code);
        }
        if self.direction_angle < 0 {
            self.direction_angle += 360;
        }
    }

    fn move_forward(&mut self) {
        match self.direction_angle {
            0 => self.y -= 1,
            90 => self.x += 1,
            180 => self.y += 1,
            270 => self.x -= 1,
            _ => panic!("Bad direction_angle {}", self.direction_angle),
        }
        println!("Moved to ({}, {})", self.x, self.y);
    }

    fn run_program(&mut self, part: u32) {
        let mut hull;
        if part == 1 {
            hull = Hull::new(200);
        } else {
            hull = Hull::new(50);
            hull.paint(0, 0, 1);
        }
        let mut hash = HashSet::new();

        loop {
            let color_under_robot = hull.color_of_plate(self.x, self.y);
            self.program.push_input(color_under_robot as i128);
            self.program = self.program.run_prog();
            if self.program.is_done() {
                break;
            }
            let color_code = self.program.get_output() as i32;
            println!("color code {}", color_code);
            hash.insert((self.x, self.y));
            hull.paint(self.x, self.y, color_code);
            self.program = self.program.run_prog();
            let direction_code = self.program.get_output() as u32;
            println!("direction code {}", direction_code);
            self.rotate(direction_code);
            self.move_forward();
        }
        if part == 1 {
            println!("number of panels painted: {}", hash.len());
        } else {
            hull.print();
        }
    }

    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn run(input: &str) {
    let mut robot = Robot::new(input);
    robot.run_program(2);
}

#[cfg(test)]
mod hull {
    use super::*;

    #[test]
    fn new() {
        let hull = Hull::new(11);
        assert_eq!(hull.width, 11);
        assert_eq!(hull.offset, 5);
        assert_eq!(hull.plates.len(), 121);
    }

    #[test]
    fn index_for() {
        let hull = Hull::new(11);
        assert_eq!(hull.index_for(-5, -5), 0);
        assert_eq!(hull.index_for(0, 0), 60);
        assert_eq!(hull.index_for(5, 5), 120);
    }

    #[test]
    fn paint() {
        let mut hull = Hull::new(11);
        assert_eq!(hull.plates[0], Color::Black);
        hull.paint(-5, -5, 1);
        assert_eq!(hull.plates[0], Color::White);
        hull.paint(-5, -5, 0);
        assert_eq!(hull.plates[0], Color::Black);
    }

    #[test]
    fn color_of_plate() {
        let mut hull = Hull::new(11);
        assert_eq!(hull.color_of_plate(-5, -5), 0);
        hull.paint(-5, -5, 1);
        assert_eq!(hull.color_of_plate(-5, -5), 1);
    }
}

#[cfg(test)]
mod robot {
    use super::*;

    #[test]
    fn new() {
        let robot = Robot::new("");
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        assert_eq!(robot.direction_angle, 0);
    }

    #[test]
    fn rotate_0() {
        let mut robot = Robot::new("");
        robot.rotate(0);
        assert_eq!(robot.direction_angle, 270);
        robot.rotate(0);
        assert_eq!(robot.direction_angle, 180);
        robot.rotate(0);
        assert_eq!(robot.direction_angle, 90);
        robot.rotate(0);
        assert_eq!(robot.direction_angle, 0);
    }

    #[test]
    fn rotate_1() {
        let mut robot = Robot::new("");
        robot.rotate(1);
        assert_eq!(robot.direction_angle, 90);
        robot.rotate(1);
        assert_eq!(robot.direction_angle, 180);
        robot.rotate(1);
        assert_eq!(robot.direction_angle, 270);
        robot.rotate(1);
        assert_eq!(robot.direction_angle, 0);
    }

    #[test]
    fn move_forward() {
        let mut robot = Robot::new("");
        robot.move_forward();
        assert_eq!(robot.get_position(), (0, -1));
        robot.rotate(1);
        robot.move_forward();
        assert_eq!(robot.get_position(), (1, -1));
        robot.rotate(1);
        robot.move_forward();
        assert_eq!(robot.get_position(), (1, 0));
        robot.rotate(1);
        robot.move_forward();
        assert_eq!(robot.get_position(), (0, 0));
        robot.move_forward();
        robot.move_forward();
        assert_eq!(robot.get_position(), (-2, 0));
    }
}
