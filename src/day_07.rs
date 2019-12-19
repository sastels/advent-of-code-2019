#[path = "intcode.rs"]
mod intcode;

use std::collections::HashSet;

fn run_amps(code: &str, settings: &[usize], input0: i32) -> i32 {
    let amp0 = intcode::Program::new(code, &format!("{}, {}", settings[0], input0));
    let output0 = amp0.run_prog().get_output();

    let amp1 = intcode::Program::new(code, &format!("{}, {}", settings[1], output0));
    let output1 = amp1.run_prog().get_output();

    let amp2 = intcode::Program::new(code, &format!("{}, {}", settings[2], output1));
    let output2 = amp2.run_prog().get_output();

    let amp3 = intcode::Program::new(code, &format!("{}, {}", settings[3], output2));
    let output3 = amp3.run_prog().get_output();

    let amp4 = intcode::Program::new(code, &format!("{}, {}", settings[4], output3));
    let output4 = amp4.run_prog().get_output();
    return output4;
}

fn run_amps_with_feedback(code: &str, settings: &[usize], input0_init: i32) -> i32 {
    let mut amp0 = intcode::Program::new(code, &format!("{}", settings[0]));
    let mut amp1 = intcode::Program::new(code, &format!("{}", settings[1]));
    let mut amp2 = intcode::Program::new(code, &format!("{}", settings[2]));
    let mut amp3 = intcode::Program::new(code, &format!("{}", settings[3]));
    let mut amp4 = intcode::Program::new(code, &format!("{}", settings[4]));

    let mut input0 = input0_init;

    loop {
        amp0.push_input(input0);
        amp0 = amp0.run_prog();
        if amp0.is_done() {
            break;
        }
        let input1 = amp0.get_output();

        amp1.push_input(input1);
        amp1 = amp1.run_prog();
        if amp1.is_done() {
            break;
        }
        let input2 = amp1.get_output();

        amp2.push_input(input2);
        amp2 = amp2.run_prog();
        if amp2.is_done() {
            break;
        }
        let input3 = amp2.get_output();

        amp3.push_input(input3);
        amp3 = amp3.run_prog();
        if amp3.is_done() {
            break;
        }
        let input4 = amp3.get_output();

        amp4.push_input(input4);
        amp4 = amp4.run_prog();
        if amp4.is_done() {
            break;
        }
        input0 = amp4.get_output();
    }
    input0
}

pub fn run_part1(input7: &str) {
    let mut max_signal = 0;
    for (p0, p1, p2, p3, p4) in iproduct!(0..5, 0..5, 0..5, 0..5, 0..5) {
        let mut phases = HashSet::new();
        phases.insert(p0);
        phases.insert(p1);
        phases.insert(p2);
        phases.insert(p3);
        phases.insert(p4);
        if phases.len() == 5 {
            let amps = run_amps(input7, &[p0, p1, p2, p3, p4], 0);
            if amps > max_signal {
                max_signal = amps;
            }
        }
    }
    println!("Max signal part 1: {}", max_signal);
}

pub fn run_part2(input7: &str) {
    let mut max_signal = 0;
    for (p0, p1, p2, p3, p4) in iproduct!(5..10, 5..10, 5..10, 5..10, 5..10) {
        let mut phases = HashSet::new();
        phases.insert(p0);
        phases.insert(p1);
        phases.insert(p2);
        phases.insert(p3);
        phases.insert(p4);
        if phases.len() == 5 {
            let amps = run_amps_with_feedback(input7, &[p0, p1, p2, p3, p4], 0);
            if amps > max_signal {
                max_signal = amps;
            }
        }
    }
    println!("Max signal part2: {}", max_signal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_amps_1() {
        let code = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let phase = [4, 3, 2, 1, 0];
        let signal = run_amps(code, &phase, 0);
        assert_eq!(signal, 43210);
    }

    #[test]
    fn run_amps_2() {
        let code = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let phase = [0, 1, 2, 3, 4];
        let signal = run_amps(code, &phase, 0);
        assert_eq!(signal, 54321);
    }

    #[test]
    fn run_amps_3() {
        let code = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let phase = [1, 0, 4, 3, 2];
        let signal = run_amps(code, &phase, 0);
        assert_eq!(signal, 65210);
    }

    #[test]
    fn run_amps_with_feedback_1() {
        let code =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let phase = [9, 8, 7, 6, 5];
        let signal = run_amps_with_feedback(code, &phase, 0);
        assert_eq!(signal, 139629729);
    }

    #[test]
    fn run_amps_with_feedback_2() {
        let code = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let phase = [9, 7, 8, 5, 6];
        let signal = run_amps_with_feedback(code, &phase, 0);
        assert_eq!(signal, 18216);
    }
}
