#[path = "intcode.rs"]
mod intcode;

pub fn run_part1(input9: &str) {
    let mut prog = intcode::Program::new(input9, "1");
    loop {
        prog = prog.run_prog();
        if prog.is_done() {
            break;
        }
        let output = prog.get_output();
        println!(" part 1: {}", output);
    }
}

pub fn run_part2(input9: &str) {
    let mut prog = intcode::Program::new(input9, "2");
    loop {
        prog = prog.run_prog();
        if prog.is_done() {
            break;
        }
        let output = prog.get_output();
        println!(" part 2: {}", output);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn run_1() {
        assert_eq!(1, 1);
    }
}
