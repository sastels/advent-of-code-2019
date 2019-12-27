use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    contains_asteroid: bool,
    angle_from_station: Option<f64>,
    distance_from_station: Option<f64>,
}

#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct AsteroidField {
    field: Vec<Position>,
    station: Option<Position>,
    num_asteroids_visible_from_station: Option<u32>,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) angle: {:?}, distance: {:?}",
            self.x, self.y, self.angle_from_station, self.distance_from_station
        )
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 0.0001 && (self.y - other.y).abs() < 0.0001
    }
}

impl Eq for Vector {}

impl Vector {
    fn hash_representation(&self) -> (i32, i32) {
        let v_len = ((self.x * self.x + self.y * self.y) as f64).sqrt();
        (
            (self.x / v_len * 10000.0) as i32,
            (self.y / v_len * 10000.0) as i32,
        )
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
            contains_asteroid: false,
            angle_from_station: None,
            distance_from_station: None,
        }
    }

    fn vector_to_position(&self, other: &Position) -> Vector {
        let vx = other.x - self.x;
        let vy = other.y - self.y;
        Vector {
            x: vx as f64,
            y: vy as f64,
        }
    }

    fn angle_to_position(&self, other: &Position) -> f64 {
        let vector_to_position = self.vector_to_position(other);
        let mut angle =
            90.0 + vector_to_position.y.atan2(vector_to_position.x) * 180.0 / std::f64::consts::PI;
        if angle < 0.0 {
            return angle + 360.0;
        } else if angle >= 360.0 {
            return angle - 360.0;
        } else {
            return angle;
        }
    }

    fn distance_to_position(&self, other: &Position) -> f64 {
        let v = self.vector_to_position(other);
        (v.x * v.x + v.y * v.y).sqrt()
    }
}

impl AsteroidField {
    pub fn new(input: &str, width: i32) -> AsteroidField {
        AsteroidField {
            field: input
                .chars()
                .enumerate()
                .map(|(i, c)| Position {
                    x: i as i32 % width,
                    y: i as i32 / width,
                    contains_asteroid: c == '#',
                    angle_from_station: None,
                    distance_from_station: None,
                })
                .collect(),
            station: None,
            num_asteroids_visible_from_station: None,
        }
    }

    fn num_asteroids_visible(&self, start: &Position) -> u32 {
        let mut hash = HashMap::new();

        for asteroid in self
            .field
            .iter()
            .filter(|p| p.contains_asteroid)
            .filter(|p| p.x != start.x || p.y != start.y)
        {
            let start_vec = start.vector_to_position(asteroid);
            hash.insert(start_vec.hash_representation(), 1);
        }
        hash.len() as u32
    }

    fn set_station(&mut self) {
        let mut max_asteroids_visible = 0;
        for asteroid in self.field.iter().filter(|p| p.contains_asteroid) {
            let num_asteroids_visible = self.num_asteroids_visible(asteroid);
            if num_asteroids_visible > max_asteroids_visible {
                max_asteroids_visible = num_asteroids_visible;
                self.station = Some(asteroid.clone());
                self.num_asteroids_visible_from_station = Some(num_asteroids_visible);
            }
        }
    }

    fn calc_angles_and_distances(&mut self) {
        let station = self.station.unwrap();
        for asteroid in self
            .field
            .iter_mut()
            .filter(|p| p.contains_asteroid)
            .filter(|p| p.x != station.x || p.y != station.y)
        {
            asteroid.angle_from_station = Some(station.angle_to_position(asteroid));
            asteroid.distance_from_station = Some(station.distance_to_position(asteroid));
        }
    }

    fn zap_asteroid_after(&mut self, angle_floor: f64) -> Option<Position> {
        let station = self.station.unwrap();
        let mut min_angle_found = 666.0;
        let mut min_distance_found = 100000.0;
        for asteroid in self
            .field
            .iter()
            .filter(|p| p.contains_asteroid)
            .filter(|p| p.x != station.x || p.y != station.y)
        {
            let angle = asteroid.angle_from_station.unwrap();
            let distance = asteroid.distance_from_station.unwrap();
            if angle > angle_floor {
                if angle < min_angle_found - 0.0001 {
                    // println!(
                    //     "{:?} new angle {} new distance {}",
                    //     asteroid, angle, distance
                    // );
                    min_angle_found = angle;
                    min_distance_found = distance;
                } else if angle == min_angle_found && (distance < min_distance_found - 0.0001) {
                    // println!("{:?} new distance {}", asteroid, distance);
                    min_distance_found = distance;
                }
            }
        }
        for asteroid in self
            .field
            .iter_mut()
            .filter(|p| p.contains_asteroid)
            .filter(|p| p.x != station.x || p.y != station.y)
        {
            if asteroid.angle_from_station.unwrap() == min_angle_found
                && asteroid.distance_from_station.unwrap() == min_distance_found
            {
                asteroid.contains_asteroid = false;
                return Some(asteroid.clone());
            }
        }
        return None;
    }

    fn zap_n_asteroids(&mut self, n: usize) -> Position {
        let mut min_angle = -1.0;
        let mut num_zapped = 0;
        let mut zapped = self.field[0].clone();

        loop {
            if num_zapped == n {
                break;
            }
            num_zapped += 1;
            let mut zapped_option = self.zap_asteroid_after(min_angle);
            if zapped_option.is_none() {
                zapped_option = self.zap_asteroid_after(-1.0);
            }
            zapped = zapped_option.unwrap();
            println!(
                "num {} {} zapped {:?}",
                num_zapped,
                (num_zapped - 1) % 9 + 1,
                zapped
            );

            min_angle = zapped.angle_from_station.unwrap();
        }
        return zapped;
    }
}

pub fn run_part1(input10: &str) {
    let mut asteroids = AsteroidField::new(input10, 33);
    asteroids.set_station();
    println!(
        "Part 1: {}",
        asteroids.num_asteroids_visible_from_station.unwrap()
    )
}

pub fn run_part2(input10: &str) {
    let mut asteroids = AsteroidField::new(input10, 33);
    asteroids.set_station();
    asteroids.calc_angles_and_distances();
    let p = asteroids.zap_n_asteroids(200);
    println!("Part 2: {:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let asteroids = AsteroidField::new(".#..#.....#####....#...##", 5);
        assert_eq!(asteroids.field.len(), 25);
        assert_eq!(asteroids.field[0].contains_asteroid, false);
        assert_eq!(asteroids.field[1].contains_asteroid, true);
        assert_eq!(asteroids.field[5].x, 0);
        assert_eq!(asteroids.field[5].y, 1);
    }

    #[test]
    fn num_asteroids_visible_1() {
        let asteroids = AsteroidField::new(".#..#.....#####....#...##", 5);
        assert_eq!(asteroids.num_asteroids_visible(&asteroids.field[1]), 7);
        assert_eq!(asteroids.num_asteroids_visible(&asteroids.field[10]), 6);
        assert_eq!(asteroids.num_asteroids_visible(&asteroids.field[14]), 5);
        assert_eq!(asteroids.num_asteroids_visible(&asteroids.field[23]), 8);
    }

    #[test]
    fn set_station_1() {
        let mut asteroids = AsteroidField::new("......#.#.#..#.#......#######..#.#.###...#..#.......#....#.##..#....#..##.#..#####...#..#..#....####", 10);
        asteroids.set_station();
        assert_eq!(asteroids.num_asteroids_visible_from_station.unwrap(), 33);
    }

    #[test]
    fn set_station_2() {
        let mut asteroids = AsteroidField::new(
            ".#....#####...#..##...##.#####..####...#...#.#####...#.....#...###....#.#.....#....##",
            17,
        );
        asteroids.set_station();
        assert_eq!(asteroids.station.unwrap().x, 8);
        assert_eq!(asteroids.station.unwrap().y, 3);
    }

    #[test]
    fn angle_to_position_0() {
        let origin = Position::new(0, 0);
        assert_eq!(origin.angle_to_position(&Position::new(0, -1)), 0.0);
    }

    #[test]
    fn angle_to_position_45() {
        let origin = Position::new(0, 0);
        assert_eq!(origin.angle_to_position(&Position::new(1, -1)), 45.0);
    }

    #[test]
    fn angle_to_position_90() {
        let origin = Position::new(0, 0);
        assert_eq!(origin.angle_to_position(&Position::new(1, 0)), 90.0);
    }

    #[test]
    fn angle_to_position_135() {
        let origin = Position::new(0, 0);
        assert_eq!(origin.angle_to_position(&Position::new(1, 1)), 135.0);
    }

    #[test]
    fn angle_to_position_180() {
        let origin = Position::new(0, 0);
        assert_eq!(origin.angle_to_position(&Position::new(0, 1)), 180.0);
    }

    #[test]
    fn angle_to_position_315() {
        let origin = Position::new(0, 0);
        assert_eq!(origin.angle_to_position(&Position::new(-1, -1)), 315.0);
    }

    #[test]
    fn angle_to_position_135_2() {
        let origin = Position::new(0, 0);
        assert_eq!(
            Position::new(1, 2).angle_to_position(&Position::new(11, 12)),
            135.0
        );
    }

    #[test]
    fn calc_angles_and_distances() {
        let mut asteroids = AsteroidField::new(".#..#.....#####....#...##", 5);
        asteroids.set_station();
        asteroids.calc_angles_and_distances();
        assert_eq!(asteroids.field[13].angle_from_station.unwrap(), 0.0);
        assert_eq!(asteroids.field[13].distance_from_station.unwrap(), 2.0);
    }

    #[test]
    fn zap_asteroid_after() {
        let mut asteroids = AsteroidField::new(
            ".#....#####...#..##...##.#####..####...#...#.#####...#.....#...###....#.#.....#....##",
            17,
        );
        asteroids.set_station();
        asteroids.calc_angles_and_distances();
        let mut p = asteroids.zap_asteroid_after(-1.0).unwrap();
        assert_eq!(p.x, 8);
        assert_eq!(p.y, 1);
        p = asteroids
            .zap_asteroid_after(p.angle_from_station.unwrap())
            .unwrap();
        assert_eq!(p.x, 9);
        assert_eq!(p.y, 0);
        p = asteroids
            .zap_asteroid_after(p.angle_from_station.unwrap())
            .unwrap();
        p = asteroids
            .zap_asteroid_after(p.angle_from_station.unwrap())
            .unwrap();
        p = asteroids
            .zap_asteroid_after(p.angle_from_station.unwrap())
            .unwrap();
        assert_eq!(p.x, 9);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn zap_n_asteroids_2() {
        let mut asteroids = AsteroidField::new(
            ".#....#####...#..##...##.#####..####...#...#.#####...#.....#...###....#.#.....#....##",
            17,
        );
        asteroids.set_station();
        asteroids.calc_angles_and_distances();
        let p = asteroids.zap_n_asteroids(2);
        assert_eq!(p.x, 9);
        assert_eq!(p.y, 0);
    }

    #[test]
    fn zap_n_asteroids_8() {
        let mut asteroids = AsteroidField::new(
            ".#....#####...#..##...##.#####..####...#...#.#####...#.....#...###....#.#.....#....##",
            17,
        );
        asteroids.set_station();
        asteroids.calc_angles_and_distances();
        let p = asteroids.zap_n_asteroids(8);
        assert_eq!(p.x, 11);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn zap_n_asteroids_30() {
        let mut asteroids = AsteroidField::new(
            ".#....#####...#..##...##.#####..####...#...#.#####...#.....#...###....#.#.....#....##",
            17,
        );
        asteroids.set_station();
        asteroids.calc_angles_and_distances();
        let p = asteroids.zap_n_asteroids(30);
        assert_eq!(p.x, 7);
        assert_eq!(p.y, 0);
    }

    #[test]
    fn zap_n_asteroids_31() {
        let mut asteroids = AsteroidField::new(
            ".#....#####...#..##...##.#####..####...#...#.#####...#.....#...###....#.#.....#....##",
            17,
        );
        asteroids.set_station();
        asteroids.calc_angles_and_distances();
        let p = asteroids.zap_n_asteroids(31);
        assert_eq!(p.x, 8);
        assert_eq!(p.y, 0);
    }
}
