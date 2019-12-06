#[derive(Clone)]
struct Program {
    code: Vec<i32>,
    inst_ptr: usize,
    done: bool,
    input: i32,
    output: i32,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {}, done: {} input: {} output: {}",
            self.code, self.inst_ptr, self.done, self.input, self.output
        )
    }
}

fn prog_from_array(arr: &[i32], input: i32) -> Program {
    Program {
        code: arr.to_vec(),
        inst_ptr: 0,
        done: false,
        input: input,
        output: -666,
    }
}

fn step_prog(prog_orig: Program) -> Program {
    let mut prog = prog_orig.clone();
    let op_full = prog.code[prog.inst_ptr];

    let op = op_full % 100;
    let mode1 = (op_full / 100) % 10;
    let mode2 = (op_full / 1000) % 10;

    let op1;
    let op2;
    let res_addr: usize;
    match op {
        1 | 2 | 7 | 8 => {
            let op1_addr = prog.code[prog.inst_ptr + 1];
            let op2_addr = prog.code[prog.inst_ptr + 2];
            res_addr = prog.code[prog.inst_ptr + 3] as usize;
            op1 = if mode1 == 0 {
                prog.code[op1_addr as usize]
            } else {
                op1_addr
            };
            op2 = if mode2 == 0 {
                prog.code[op2_addr as usize]
            } else {
                op2_addr
            };
        }
        3 | 4 => {
            op1 = 0;
            op2 = 0;
            res_addr = prog.code[prog.inst_ptr + 1] as usize;
        }
        5 | 6 => {
            res_addr = 0;
            let op1_addr = prog.code[prog.inst_ptr + 1];
            let op2_addr = prog.code[prog.inst_ptr + 2];
            op1 = if mode1 == 0 {
                prog.code[op1_addr as usize]
            } else {
                op1_addr
            };
            op2 = if mode2 == 0 {
                prog.code[op2_addr as usize]
            } else {
                op2_addr
            };
        }
        _ => {
            op1 = 0;
            op2 = 0;
            res_addr = 0;
        }
    }

    match op {
        1 => {
            prog.code[res_addr] = op1 + op2;
            prog.inst_ptr += 4;
        }
        2 => {
            prog.code[res_addr] = op1 * op2;
            prog.inst_ptr += 4;
        }
        3 => {
            prog.code[res_addr] = prog.input;
            prog.inst_ptr += 2;
        }
        4 => {
            prog.output = prog.code[res_addr];
            prog.inst_ptr += 2;
        }
        5 => {
            if op1 != 0 {
                prog.inst_ptr = op2 as usize;
            } else {
                prog.inst_ptr += 3;
            }
        }
        6 => {
            if op1 == 0 {
                prog.inst_ptr = op2 as usize;
            } else {
                prog.inst_ptr += 3;
            }
        }
        7 => {
            prog.code[res_addr] = if op1 < op2 { 1 } else { 0 };
            prog.inst_ptr += 4;
        }
        8 => {
            prog.code[res_addr] = if op1 == op2 { 1 } else { 0 };
            prog.inst_ptr += 4;
        }
        99 => {
            prog.done = true;
        }
        _ => {
            println!("Invalid opcode: {}!", op);
            prog.done = true;
        }
    }
    return prog;
}

fn run_prog(prog_orig: Program) -> Program {
    let mut prog = prog_orig.clone();
    loop {
        if prog.done {
            break;
        }
        prog = step_prog(prog);
    }
    return prog;
}

fn run_diagnostic(prog_orig: Program) -> i32 {
    run_prog(prog_orig).output
}

pub fn day5a(inp_array: &[i32]) {
    println!(
        "Diagnostic A: {}",
        run_diagnostic(prog_from_array(inp_array, 1))
    );

    println!(
        "Diagnostic B: {}",
        run_diagnostic(prog_from_array(inp_array, 5))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_step_prog_1() {
        let inst = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let prog = step_prog(prog_from_array(&inst, 0));
        assert_eq!(prog.done, false);
        assert_eq!(prog.code[3], 70);
    }

    #[test]
    fn test_run_prog_orig_1() {
        let prog = run_prog(prog_from_array(&[1, 1, 1, 4, 99, 5, 6, 0, 99], 0));
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[0], 30);
    }

    #[test]
    fn test_run_prog_orig_2() {
        let prog = run_prog(prog_from_array(
            &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            0,
        ));
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[0], 3500);
    }

    #[test]
    fn test_run_prog_input() {
        let prog = run_prog(prog_from_array(&[3, 3, 99, 11], 666));
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[3], 666);
    }

    #[test]
    fn test_run_prog_output() {
        let prog = run_prog(prog_from_array(&[3, 0, 4, 0, 99], 666));
        assert_eq!(prog.done, true);
        assert_eq!(prog.output, 666);
    }

    #[test]
    fn test_run_prog_modes() {
        let prog = run_prog(prog_from_array(&[1002, 4, 3, 4, 33], 666));
        assert_eq!(prog.done, true);
        assert_eq!(prog.inst_ptr, 4);
    }

    #[test]
    fn test_full_1a() {
        let out = run_diagnostic(prog_from_array(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8));
        assert_eq!(out, 1);
    }

    #[test]
    fn test_full_1b() {
        let out = run_diagnostic(prog_from_array(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], 11));
        assert_eq!(out, 0);
    }
    #[test]
    fn test_full_1c() {
        let out = run_diagnostic(prog_from_array(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], 2));
        assert_eq!(out, 1);
    }
}
