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

    pub fn reaction_for_output(&self, output: &String) -> Option<&Reaction> {
        self.reactions.get(output)
    }

    pub fn ore_for_fuel(&self) -> u32 {
        // find the starting list of needed ingredients

        let mut needed = self
            .reaction_for_output(&"FUEL".to_string())
            .unwrap()
            .clone()
            .inputs;
        println!("needed: {:?}", needed);

        // find the next one we can create

        let mut next_choice_option = None;
        for ingredient in needed.iter().map(|i| i.clone()) {
            if !needed
                .iter()
                .map(|i| i.name.clone())
                .map(|name| self.reaction_for_output(&name).unwrap())
                .any(|r| r.inputs.iter().any(|ii| ii.name == ingredient.name))
            {
                next_choice_option = Some(ingredient.clone());
                break;
            }
        }
        let next_choice = next_choice_option.unwrap(); // will panic if didn't find one
        println!("finding {:?}", next_choice);

        // take this one out of the needed list, and add in all the ingredients from its reaction
        // (with correct numbers)

        needed = needed
            .iter()
            .filter(|i| i.name != next_choice.name)
            .map(|ir| ir.clone())
            .collect();

        let next_choice_reaction = self.reactions.get(&next_choice.name).unwrap();

        for ingredient in next_choice_reaction.inputs.iter() {
            let mut needed_ingredient = ingredient.clone();
            let multiplier = (next_choice.number + next_choice_reaction.output.number - 1)
                / next_choice_reaction.output.number;
            needed_ingredient.number *= multiplier;
            needed.push(needed_ingredient);
        }
        println!("now needed: {:?}", needed);

        // collapse the needed list to put together the same ingredients

        0
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
        let input = "10 ORE => 10 A\n7 A, 1 D => 1 E";
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

    #[test]
    fn reaction_for_output() {
        let rl = ReactionList::new("10 ORE => 10 A\n7 A, 1 D => 1 E");
        assert_eq!(
            *rl.reaction_for_output(&"A".to_string()).unwrap(),
            Reaction::new("10 ORE => 10 A")
        );
        assert_eq!(rl.reaction_for_output(&"NONO".to_string()).is_none(), true);
    }

    #[test]
    fn ore_for_fuel() {
        let input = "
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
        ";
        let rl = ReactionList::new(input);
        assert_eq!(rl.ore_for_fuel(), 31);
    }
}
