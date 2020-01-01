#[path = "intcode.rs"]
mod intcode;

use std::collections::HashSet;

struct Screen {
    tiles: Vec<u32>,
    width: usize,
}

struct Game {
    screen: Screen,
    program: intcode::Program,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen {
            tiles: vec![0; width * height],
            width: width,
        }
    }

    fn index_for(&self, x: u32, y: u32) -> usize {
        self.width * (y as usize) + (x as usize)
    }

    fn xy_for(&self, index: usize) -> u32 {
        (index % self.width) as u32
    }

    fn set_tile(&mut self, x: u32, y: u32, tile_id: u32) {
        // println!("Set tile {} {} to {}", x, y, tile_id);
        let index = self.index_for(x, y);
        self.tiles[index] = tile_id;
    }

    fn id_of_tile(&self, x: u32, y: u32) -> u32 {
        self.tiles[self.index_for(x, y)]
    }

    fn ball_x(&self) -> u32 {
        for (index, id) in self.tiles.iter().enumerate() {
            if *id == 4 {
                return self.xy_for(index);
            }
        }
        panic!("Couldn't find ball!")
    }

    fn paddle_x(&self) -> u32 {
        for (index, id) in self.tiles.iter().enumerate() {
            if *id == 3 {
                return self.xy_for(index);
            }
        }
        panic!("Couldn't find paddle!")
    }

    fn print(&self) {
        for (i, id) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {
                println!();
            }
            match id {
                0 => print!(" "),
                1 => print!("|"),
                2 => print!("X"),
                3 => print!("-"),
                4 => print!("O"),
                _ => print!("{}", id),
            }
        }
        println!();
    }
}

impl Game {
    fn new(code: &str, screen_width: usize, screen_height: usize) -> Game {
        Game {
            screen: Screen::new(screen_width, screen_height),
            program: intcode::Program::new(code, ""),
        }
    }

    fn add_quarter(&mut self) {
        self.program.poke(0, 2);
    }

    fn play(&mut self) {
        let mut num_score_bumps = 0;
        loop {
            self.program = self.program.run_prog();
            if self.program.is_done() {
                break;
            } else if self.program.needs_input() {
                let ball_x = self.screen.ball_x();
                let paddle_x = self.screen.paddle_x();
                if paddle_x < ball_x {
                    self.program.push_input(1);
                } else if paddle_x > ball_x {
                    self.program.push_input(-1);
                } else {
                    self.program.push_input(0);
                }
            } else {
                let x = self.program.get_output() as i32;
                self.program = self.program.run_prog();
                let y = self.program.get_output() as u32;
                self.program = self.program.run_prog();
                let tile_id = self.program.get_output() as u32;
                if (x, y) == (-1, 0) {
                    num_score_bumps += 1;
                    println!(
                        "********** {} Score: {}  **********",
                        num_score_bumps, tile_id
                    );
                } else {
                    self.screen.set_tile(x as u32, y, tile_id);
                }
            }
        }
        self.screen.print();
    }
}
pub fn step_1(input: &str) {
    let mut screen = Screen::new(50, 50);
    let mut hash = HashSet::new();
    let mut program = intcode::Program::new(input, "");

    loop {
        program = program.run_prog();
        if program.is_done() {
            break;
        }
        let x = program.get_output() as u32;
        program = program.run_prog();
        let y = program.get_output() as u32;
        program = program.run_prog();
        let tile_id = program.get_output() as u32;

        println!("{} {} : {}", x, y, tile_id);
        // count block tiles
        if tile_id == 2 {
            hash.insert((x, y));
        }
    }
    println!("number of block tiles: {}", hash.len());
}

pub fn step_2(input: &str) {
    let mut game = Game::new(input, 41, 25);

    game.add_quarter();
    game.play();
}

#[cfg(test)]
mod screen {
    use super::*;
}
