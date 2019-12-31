use regex::Regex;

#[derive(Clone, PartialEq, Eq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

#[derive(Clone, PartialEq, Eq)]
struct Moon1D {
    p: i32,
    v: i32,
}

#[derive(Clone)]
struct System {
    moons: Vec<Moon>,
}

#[derive(Clone)]
struct System1D {
    moons: Vec<Moon1D>,
}

impl std::fmt::Debug for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "pos: ({}, {}, {}) vel: ({}, {}, {})",
            self.x, self.y, self.z, self.vx, self.vy, self.vz
        )
    }
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x: x,
            y: y,
            z: z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn new_from_str(s: &str) -> Moon {
        let re = Regex::new(r"<x=(.+), y=(.+), z=(.+)>").unwrap();
        let cap = re.captures(s).unwrap();
        Moon::new(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap(),
        )
    }

    fn position(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    fn velocity(&self) -> (i32, i32, i32) {
        (self.vx, self.vy, self.vz)
    }

    fn update_velocity_wrt(&mut self, m: &Moon) {
        if self.x != m.x {
            self.vx += if self.x < m.x { 1 } else { -1 };
        }
        if self.y != m.y {
            self.vy += if self.y < m.y { 1 } else { -1 };
        }
        if self.z != m.z {
            self.vz += if self.z < m.z { 1 } else { -1 };
        }
    }

    fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn total_energy(&self) -> i32 {
        let potential_energy = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic_energy = self.vx.abs() + self.vy.abs() + self.vz.abs();
        potential_energy * kinetic_energy
    }
}

impl std::fmt::Debug for System {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.moons)
    }
}

impl System {
    fn new() -> System {
        System { moons: Vec::new() }
    }

    fn add_moon_from_str(&mut self, s: &str) {
        self.moons.push(Moon::new_from_str(s));
    }

    fn add_moon(&mut self, x: i32, y: i32, z: i32) {
        self.moons.push(Moon::new(x, y, z));
    }

    fn display(&self) {
        for m in self.moons.iter() {
            println!("{:?}", m)
        }
    }

    fn step_velocity(&mut self) {
        let moons_copy = self.moons.clone();
        for m1 in self.moons.iter_mut() {
            for m2 in moons_copy.iter() {
                m1.update_velocity_wrt(m2);
            }
        }
    }

    fn step_position(&mut self) {
        for m in self.moons.iter_mut() {
            m.update_position();
        }
    }

    fn step(&mut self) {
        self.step_velocity();
        self.step_position();
    }

    fn run_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }

    fn equals(&self, other: &System) -> bool {
        // assumes moons Vecs are of len 4
        self.moons[0] == other.moons[0]
            && self.moons[1] == other.moons[1]
            && self.moons[2] == other.moons[2]
            && self.moons[3] == other.moons[3]
    }

    fn brent(&self) -> (u64, u64) {
        // find the length of the loop
        let mut power: u64 = 1;
        let mut lam: u64 = 1;
        let mut tortoise = self.clone();
        let mut hare = self.clone();
        hare.step();
        while !(tortoise.equals(&hare)) {
            if power == lam {
                tortoise = hare.clone();
                power *= 2;
                println!("power {}", power);
                lam = 0;
            }
            hare.step();
            lam += 1;
        }
        // find the start of the loop
        tortoise = self.clone();
        hare = self.clone();
        for _ in 0..lam {
            hare.step();
        }
        let mut mu = 0;
        while !(tortoise.equals(&hare)) {
            tortoise.step();
            hare.step();
            mu += 1;
        }
        (lam, mu)
    }

    fn loop_len(&self) -> u64 {
        let mut s1 = self.clone();
        let mut s2 = self.clone();
        s1.step();
        s2.step();
        s2.step();

        let mut len: u64 = 0;
        loop {
            if s1.equals(&s2) {
                break;
            } else {
                s1.step();
                s2.step();
                s2.step();
                if len % 10000 == 0 {
                    println!("first trip, len {}", len)
                }
                len += 1;
            }
        }
        // in the loop! now let's find the length

        let mut len: u64 = 1;
        s2.step();
        loop {
            if s1.equals(&s2) {
                break;
            } else {
                s2.step();
                len += 1;
            }
        }
        len
    }
}

impl Moon1D {
    fn new(p: i32) -> Moon1D {
        Moon1D { p: p, v: 0 }
    }
}

impl System1D {
    fn new() -> System1D {
        System1D { moons: Vec::new() }
    }

    fn add_moon(&mut self, p: i32) {
        self.moons.push(Moon1D::new(p));
    }

    fn step(&mut self) {
        for i in 0..4 {
            for j in (i + 1)..4 {
                if self.moons[i].p < self.moons[j].p {
                    self.moons[i].v += 1;
                    self.moons[j].v -= 1;
                } else if self.moons[i].p > self.moons[j].p {
                    self.moons[i].v -= 1;
                    self.moons[j].v += 1;
                }
            }
        }
        for i in 0..4 {
            self.moons[i].p += self.moons[i].v
        }
    }

    fn equals(&self, other: &System1D) -> bool {
        // assumes moons Vecs are of len 4
        self.moons[0] == other.moons[0]
            && self.moons[1] == other.moons[1]
            && self.moons[2] == other.moons[2]
            && self.moons[3] == other.moons[3]
    }

    fn brent(&self) -> (u64, u64) {
        // find the length of the loop
        let mut power: u64 = 1;
        let mut lam: u64 = 1;
        let mut tortoise = self.clone();
        let mut hare = self.clone();
        hare.step();
        while !(tortoise.equals(&hare)) {
            if power == lam {
                tortoise = hare.clone();
                power *= 2;
                lam = 0;
            }
            hare.step();
            lam += 1;
        }
        // find the start of the loop
        tortoise = self.clone();
        hare = self.clone();
        for _ in 0..lam {
            hare.step();
        }
        let mut mu = 0;
        while !(tortoise.equals(&hare)) {
            tortoise.step();
            hare.step();
            mu += 1;
        }
        (lam, mu)
    }
}

pub fn step_2() {
    let mut s = System1D::new();
    s.add_moon(-8);
    s.add_moon(5);
    s.add_moon(2);
    s.add_moon(9);
    println!("xx lambda, mu = {:?}", s.brent());

    s = System1D::new();
    s.add_moon(-10);
    s.add_moon(5);
    s.add_moon(-7);
    s.add_moon(-8);
    println!("yy lambda, mu = {:?}", s.brent());

    s = System1D::new();
    s.add_moon(0);
    s.add_moon(10);
    s.add_moon(3);
    s.add_moon(-3);
    println!("zz lambda, mu = {:?}", s.brent());

    //////
    ///
    let mut s = System1D::new();
    s.add_moon(13);
    s.add_moon(8);
    s.add_moon(-5);
    s.add_moon(2);
    println!("xxx lambda, mu = {:?}", s.brent());

    let mut s = System1D::new();
    s.add_moon(9);
    s.add_moon(14);
    s.add_moon(4);
    s.add_moon(-6);
    println!("yyy lambda, mu = {:?}", s.brent());

    let mut s = System1D::new();
    s.add_moon(5);
    s.add_moon(-2);
    s.add_moon(11);
    s.add_moon(1);
    println!("zzz lambda, mu = {:?}", s.brent());
}
//  real input
// s.add_moon_from_str("<x=13, y=9, z=5>");
// s.add_moon_from_str("<x=8, y=14, z=-2>");
// s.add_moon_from_str("<x=-5, y=4, z=11>");
// s.add_moon_from_str("<x=2, y=-6, z=1>");

pub fn step_1() {
    let mut s = System::new();
    s.add_moon_from_str("<x=-8, y=0, z=0>");
    s.add_moon_from_str("<x=5, y=0, z=0>");
    s.add_moon_from_str("<x=2, y=0, z=0>");
    s.add_moon_from_str("<x=9, y=0, z=0>");
    println!("x lambda, mu = {:?}", s.brent());

    // s.add_moon_from_str("<x=0, y=-10, z=0>");
    // s.add_moon_from_str("<x=0, y=5, z=0>");
    // s.add_moon_from_str("<x=0, y=-7, z=0>");
    // s.add_moon_from_str("<x=0, y=-8, z=-0>");
    // println!("ly yambda, mu = {:?}", s.brent());

    // s.add_moon_from_str("<x=0, y=0, z=0>");
    // s.add_moon_from_str("<x=0, y=0, z=10>");
    // s.add_moon_from_str("<x=0, y=0, z=3>");
    // s.add_moon_from_str("<x=0, y=-0, z=-3>");
    // println!("z lambda, mu = {:?}", s.brent());

    // let mut s = System::new();
    // s.add_moon_from_str("<x=13, y=9, z=5>");
    // s.add_moon_from_str("<x=8, y=14, z=-2>");
    // s.add_moon_from_str("<x=-5, y=4, z=11>");
    // s.add_moon_from_str("<x=2, y=-6, z=1>");
    // // s.run_steps(1000);
    // // println!("Part 1: total energy = {}", s.total_energy());

    // println!("Part 2: loop len {}", s.loop_len());
}

#[cfg(test)]
mod moon {
    use super::*;

    #[test]
    fn new() {
        let m = Moon::new(1, 2, 3);
        assert_eq!(m.x, 1);
        assert_eq!(m.y, 2);
        assert_eq!(m.z, 3);
        assert_eq!(m.vx, 0);
        assert_eq!(m.vy, 0);
        assert_eq!(m.vz, 0);
    }

    #[test]
    fn new_from_str() {
        let m = Moon::new_from_str("<x=2, y=-10, z=-7>");
        assert_eq!(m.x, 2);
        assert_eq!(m.y, -10);
        assert_eq!(m.z, -7);
    }

    #[test]
    fn position() {
        let m = Moon::new_from_str("<x=2, y=-10, z=-7>");
        assert_eq!(m.position(), (2, -10, -7))
    }

    #[test]
    fn velocity() {
        let mut m = Moon::new_from_str("<x=2, y=-10, z=-7>");
        m.vx = -1;
        m.vy = 22;
        m.vz = 33;
        assert_eq!(m.velocity(), (-1, 22, 33))
    }

    #[test]
    fn update_velocity_wrt() {
        let mut m1 = Moon::new(1, 2, 3);
        m1.vx = 10;
        m1.vy = 20;
        m1.vz = 30;
        let m2 = Moon::new(0, 2, 30);
        m1.update_velocity_wrt(&m2);
        assert_eq!(m1.velocity(), (9, 20, 31));
    }

    #[test]
    fn update_position() {
        let mut m = Moon::new(1, 2, 3);
        m.vx = 10;
        m.vy = 20;
        m.vz = 30;
        m.update_position();
        assert_eq!(m.position(), (11, 22, 33));
    }

    #[test]
    fn total_energy() {
        let mut m = Moon::new(1, -2, 3);
        m.vx = -2;
        m.vy = 10;
        m.vz = -5;
        assert_eq!(m.total_energy(), (1 + 2 + 3) * (2 + 10 + 5));
    }
}

#[cfg(test)]
mod system {
    use super::*;

    #[test]
    fn new() {
        let s = System::new();
        assert_eq!(s.moons.len(), 0);
    }

    #[test]
    fn add_moon() {
        let mut s = System::new();
        s.add_moon(1, 2, 3);
        s.add_moon(11, 22, 33);
        assert_eq!(s.moons[1].position(), (11, 22, 33));
    }

    #[test]
    fn add_moon_from_str() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        assert_eq!(s.moons[1].position(), (2, -10, -7));
    }

    #[test]
    fn step_velocity() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        s.add_moon_from_str("<x=4, y=-8, z=8>");
        s.add_moon_from_str("<x=3, y=5, z=-1>");
        s.step_velocity();
        assert_eq!(s.moons[0].velocity(), (3, -1, -1));
        assert_eq!(s.moons[1].velocity(), (1, 3, 3));
        assert_eq!(s.moons[2].velocity(), (-3, 1, -3));
        assert_eq!(s.moons[3].velocity(), (-1, -3, 1));
    }

    #[test]
    fn step_position() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        s.add_moon_from_str("<x=4, y=-8, z=8>");
        s.add_moon_from_str("<x=3, y=5, z=-1>");
        s.step_velocity();
        s.step_position();
        assert_eq!(s.moons[0].position(), (2, -1, 1));
        assert_eq!(s.moons[1].position(), (3, -7, -4));
        assert_eq!(s.moons[2].position(), (1, -7, 5));
        assert_eq!(s.moons[3].position(), (2, 2, 0));
    }

    #[test]
    fn run_steps() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        s.add_moon_from_str("<x=4, y=-8, z=8>");
        s.add_moon_from_str("<x=3, y=5, z=-1>");
        s.run_steps(10);
        assert_eq!(s.moons[0].position(), (2, 1, -3));
        assert_eq!(s.moons[1].position(), (1, -8, 0));
        assert_eq!(s.moons[2].position(), (3, -6, 1));
        assert_eq!(s.moons[3].position(), (2, 0, 4));
        assert_eq!(s.moons[0].velocity(), (-3, -2, 1));
    }

    #[test]
    fn total_energy() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        s.add_moon_from_str("<x=4, y=-8, z=8>");
        s.add_moon_from_str("<x=3, y=5, z=-1>");
        s.run_steps(10);
        assert_eq!(s.total_energy(), 179);
    }

    #[test]
    fn equals() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        s.add_moon_from_str("<x=4, y=-8, z=8>");
        s.add_moon_from_str("<x=3, y=5, z=-1>");

        assert_eq!(s.equals(&(s.clone())), true);
        let mut s2 = s.clone();
        s2.step();
        assert_eq!(s.equals(&s2), false);
    }

    #[test]
    fn loop_len_1() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        s.add_moon_from_str("<x=4, y=-8, z=8>");
        s.add_moon_from_str("<x=3, y=5, z=-1>");
        let len = s.loop_len();
        assert_eq!(len, 2772);
    }

    #[test]
    fn brent_1() {
        let mut s = System::new();
        s.add_moon_from_str("<x=-1, y=0, z=2>");
        s.add_moon_from_str("<x=2, y=-10, z=-7>");
        s.add_moon_from_str("<x=4, y=-8, z=8>");
        s.add_moon_from_str("<x=3, y=5, z=-1>");
        assert_eq!(s.brent(), (2772, 0));
    }
}
