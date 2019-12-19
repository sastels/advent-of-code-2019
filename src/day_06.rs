#[path = "tree.rs"]
mod tree;
use math::round;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Orbit {
    center: String,
    planet: String,
}

impl Orbit {
    fn new_from_str(s: &str) -> Orbit {
        let s_vec: Vec<&str> = s.split(')').collect();
        Orbit {
            center: s_vec[0].to_string(),
            planet: s_vec[1].to_string(),
        }
    }
}

impl std::fmt::Debug for Orbit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}){}", self.center, self.planet)
    }
}

fn orbit_vec_from_str(s: &str) -> Vec<Orbit> {
    s.split(",").map(|s| Orbit::new_from_str(s)).collect()
}

fn load_orbit_into_system(orbit_str: &str) -> tree::Node {
    let mut orbits = orbit_vec_from_str(orbit_str);
    let mut system = tree::Node::new("COM".to_string());
    // let mut new_orbits: Vec<Orbit> = Vec::new();

    loop {
        let new_orbits: Vec<Orbit> = orbits
            .iter()
            .filter(|orbit| !system.new_child_of(orbit.center.clone(), orbit.planet.clone()))
            .cloned()
            .collect();

        println!("{}", new_orbits.len());
        orbits = new_orbits;
        if orbits.len() == 0 {
            break;
        }
    }
    system
}

fn score_system(base_score: usize, system: &tree::Node) -> usize {
    let mut retval = base_score;
    for child in system.get_children() {
        retval += score_system(base_score + 1, child);
    }
    retval
}

pub fn run(data_str: &str) {
    let system = load_orbit_into_system(data_str);
    println!("part 1: {}", score_system(0, &system));
}

pub fn compute_xfer_path_len(system: &tree::Node, a_str: &str, b_str: &str) -> usize {
    let path_a = system.find_node(a_str).unwrap().get_path();
    let path_b = system.find_node(b_str).unwrap().get_path();
    let common_len = path_a
        .chars()
        .zip(path_b.chars())
        .filter(|(a, b)| *a == *b) // this isn't quite right - I think it's matching more than common prefix
        .fold(0, |acc, _| acc + 1);
    let xfer_str_len = path_a.len() + path_b.len() - 2 * common_len;
    return round::ceil(xfer_str_len as f64 / 4f64, 0) as usize;
}

pub fn run2(data_str: &str) {
    // let input_06 = "COM)B,B)C,C)D,D)E,E)F,B)G,G)H,D)I,E)J,J)K,K)L";
    let system = load_orbit_into_system(data_str);
    let san_path = system.find_node("SAN").unwrap().get_path();
    let you_path = system.find_node("YOU").unwrap().get_path();

    // println!("SAN: {}", san_path);
    // println!("YOU: {}", you_path);
    println!("part 2: {}", compute_xfer_path_len(&system, "SAN", "YOU")); // real answer is 412
}

#[cfg(test)]
mod tests {
    use super::Orbit;

    #[test]
    fn orbit_new_1() {
        let orbit = Orbit::new_from_str("a)b");
        assert_eq!(orbit.center, "a".to_string());
        assert_eq!(orbit.planet, "b".to_string());
    }

    #[test]
    fn orbit_vec_from_str() {
        let input = "B)A,COM)B";
        let orbits = super::orbit_vec_from_str(&input);
        assert_eq!(format!("{:?}", orbits[0]), "B)A");
        assert_eq!(format!("{:?}", orbits[1]), "COM)B");
    }

    #[test]
    fn load_orbit_into_system() {
        let input = "COM)b";
        let system = super::load_orbit_into_system(input);
        assert_eq!(system.get_elem(), "COM".to_string());
        assert_eq!(system.get_children()[0].get_elem(), "b".to_string());
    }

    #[test]
    fn load_orbit_into_system_2() {
        let input = "B)A,COM)B";
        let system = super::load_orbit_into_system(input);
        assert_eq!(system.get_elem(), "COM".to_string());
        assert_eq!(system.get_children()[0].get_elem(), "B".to_string());
        assert_eq!(
            system.get_children()[0].get_children()[0].get_elem(),
            "A".to_string()
        );
    }

    #[test]
    fn score_system() {
        let input = "COM)B,B)C,C)D,D)E,E)F,B)G,G)H,D)I,E)J,J)K,K)L";
        let system = super::load_orbit_into_system(input);
        assert_eq!(super::score_system(0, &system), 42);
    }

    #[test]
    fn compute_xfer_path_len() {
        let system = super::load_orbit_into_system("COM)AAA,AAA)BBB,AAA)CCC");
        assert_eq!(super::compute_xfer_path_len(&system, "BBB", "CCC"), 0);

        let system2 = super::load_orbit_into_system("COM)AAA,AAA)BBB,AAA)CCC,CCC)DDD");
        assert_eq!(super::compute_xfer_path_len(&system2, "BBB", "DDD"), 1);

        let system2 =
            super::load_orbit_into_system("COM)AAA,AAA)BBB,AAA)CCC,CCC)DDD,BBB)EEE,BBB)FFF");
        assert_eq!(super::compute_xfer_path_len(&system2, "EEE", "CCC"), 1);
        assert_eq!(super::compute_xfer_path_len(&system2, "EEE", "DDD"), 2);

        let system3 = super::load_orbit_into_system(
            "COM)BBB,BBB)CCC,CCC)DDD,DDD)EEE,EEE)FFF,BBB)GGG,GGG)HHH,DDD)III,EEE)JJJ,JJJ)KKK,KKK)LLL,KKK)YOU,III)SAN",
        );
        assert_eq!(super::compute_xfer_path_len(&system3, "YOU", "SAN"), 4);
    }
}
