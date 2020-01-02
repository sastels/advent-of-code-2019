use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct Ingredient {
    number: u32,
    name: String,
}

#[derive(Clone, Eq, PartialEq)]
struct Reaction {
    output: Ingredient,
    inputs: Vec<Ingredient>,
}

#[derive(Clone, Eq, PartialEq)]
struct ReactionList {
    reactions: HashMap<String, Reaction>,
}

impl std::fmt::Debug for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.number, self.name)
    }
}

impl std::fmt::Debug for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.inputs, self.output)
    }
}

impl std::fmt::Debug for ReactionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.reactions)
    }
}

impl Ingredient {
    pub fn new(number: u32, name: &str) -> Ingredient {
        Ingredient {
            number: number,
            name: name.to_string(),
        }
    }
}

impl Reaction {
    pub fn new(s: &str) -> Reaction {
        let re = Regex::new(r"(\d+).*?([[:alpha:]]+)").unwrap();

        let mut ingredients: Vec<Ingredient> = re
            .captures_iter(s)
            .map(|c| Ingredient::new(c[1].parse().unwrap(), &(c[2])))
            .collect();

        let output = ingredients.pop().unwrap();
        Reaction {
            output: output,
            inputs: ingredients.clone(),
        }
    }
}

impl ReactionList {
    pub fn new(reaction_lines: &str) -> ReactionList {
        let mut rl = ReactionList {
            reactions: HashMap::new(),
        };
        for rs in reaction_lines
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            rl.insert_reaction(&Reaction::new(rs));
        }
        rl
    }

    pub fn insert_reaction(&mut self, r: &Reaction) {
        self.reactions.insert(r.output.name.clone(), r.clone());
    }
}

pub fn step_1() {
    let mut rl = ReactionList::new("");
    rl.insert_reaction(&Reaction::new("3 A, 4 B => 1 AB"));
    rl.insert_reaction(&Reaction::new("3 AC, 4 BB => 10 AAB"));
    println!("{:?}", rl);
}

#[cfg(test)]
mod ingredient {
    use super::*;
}

#[cfg(test)]
mod reaction {
    use super::*;

    #[test]
    fn new() {
        let r = Reaction::new("3 AA, 4 B => 1 AB");
        assert_eq!(r.output, Ingredient::new(1, "AB"));
        assert_eq!(r.inputs.len(), 2);
        assert_eq!(r.inputs[0], Ingredient::new(3, "AA"));
        assert_eq!(r.inputs[1], Ingredient::new(4, "B"));
    }
}

#[cfg(test)]
mod reaction_list {
    use super::*;

    #[test]
    fn new_0() {
        let rl = ReactionList::new("");
        assert_eq!(rl.reactions.len(), 0);
    }

    #[test]
    fn new_2() {
        let input = "10 ORE => 10 A
        7 A, 1 D => 1 E";

        let rl = ReactionList::new(input);
        assert_eq!(
            *rl.reactions.get("A").unwrap(),
            Reaction::new("10 ORE => 10 A")
        );
        assert_eq!(
            *rl.reactions.get("E").unwrap(),
            Reaction::new("7 A, 1 D => 1 E")
        );
    }

    #[test]
    fn new_3() {
        let input = "
        
        10 ORE => 10 A
        
        7 A, 1 D => 1 E
        
               
        ";
        let rl = ReactionList::new(input);
        assert_eq!(rl.reactions.len(), 2);
        assert_eq!(
            *rl.reactions.get("A").unwrap(),
            Reaction::new("10 ORE => 10 A")
        );
        assert_eq!(
            *rl.reactions.get("E").unwrap(),
            Reaction::new("7 A, 1 D => 1 E")
        );
    }

    #[test]
    fn insert() {
        let mut rl = ReactionList::new("");
        let r1 = Reaction::new("3 A, 4 B => 1 C");
        let r2 = Reaction::new("33 AAA, 44 BB => 1 CC");
        rl.insert_reaction(&r1);
        rl.insert_reaction(&r2);
        assert_eq!(*rl.reactions.get("C").unwrap(), r1);
        assert_eq!(*rl.reactions.get("CC").unwrap(), r2);
    }
}
