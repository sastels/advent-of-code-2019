#[derive(Clone)]
pub struct Program {
    code: Vec<i32>,
    inst_ptr: usize,
    done: bool,
    input: Vec<i32>,
    input_ptr: usize,
    output: Option<i32>,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {}, done: {} input: {:?} output: {}",
            self.code,
            self.inst_ptr,
            self.done,
            self.input,
            self.output.unwrap_or(-6666)
        )
    }
}

impl Program {
    pub fn new(code: &str, input: &str) -> Program {
        let code_vec = code
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let input_vec = input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        Program::new_from_vecs(&code_vec, &input_vec)
    }

    fn new_from_vecs(arr: &Vec<i32>, input: &Vec<i32>) -> Program {
        Program {
            code: arr.clone(),
            inst_ptr: 0,
            done: false,
            input: input.clone(),
            input_ptr: 0,
            output: None,
        }
    }

    pub fn push_input(&mut self, input: i32) {
        self.input.push(input)
    }

    pub fn get_output(&mut self) -> i32 {
        let output = self.output;
        self.output = None;
        output.unwrap()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    fn step_prog(&self) -> Program {
        let mut prog = self.clone();
        prog.done = false;
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
                prog.code[res_addr] = prog.input[prog.input_ptr];
                prog.inst_ptr += 2;
                prog.input_ptr += 1;
            }
            4 => {
                prog.output = Some(prog.code[res_addr]);
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

    pub fn run_prog(&self) -> Program {
        let mut prog = self.clone();
        loop {
            if prog.done || prog.output.is_some() {
                break;
            }
            prog = prog.step_prog();
        }
        return prog;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_prog() {
        let inst = "1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50";
        let prog = Program::new(inst, "0");
        let prog = prog.step_prog();
        assert_eq!(prog.done, false);
        assert_eq!(prog.code[3], 70);
    }

    #[test]
    fn run_prog_1() {
        let prog = Program::new("1, 1, 1, 4, 99, 5, 6, 0, 99", "0");
        let prog = prog.run_prog();
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[0], 30);
    }

    #[test]
    fn run_prog_2() {
        let prog = Program::new("1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50", "0");
        let prog = prog.run_prog();
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[0], 3500);
    }

    #[test]
    fn run_prog_input() {
        let prog = Program::new("3, 3, 99, 11", "666");
        let prog = prog.run_prog();
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[3], 666);
    }

    #[test]
    fn run_prog_2_inputs() {
        let prog = Program::new("3, 9, 3, 10, 1, 9, 10, 0, 99, 99, 99, 99, 99", "222, 111");
        let prog = prog.run_prog();
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[0], 333);
    }

    #[test]
    fn push_input() {
        let mut prog = Program::new("3, 3, 99, 11", "");
        prog.push_input(666);
        prog = prog.run_prog();
        assert_eq!(prog.done, true);
        assert_eq!(prog.code[3], 666);
    }

    #[test]
    fn run_prog_output() {
        let prog = Program::new("3, 0, 4, 0, 99", "666");
        let prog = prog.run_prog();
        assert_eq!(prog.output, Some(666));
    }

    #[test]
    fn run_prog_modes() {
        let prog = Program::new("1002, 4, 3, 4, 33", "666");
        let prog = prog.run_prog();
        assert_eq!(prog.done, true);
        assert_eq!(prog.inst_ptr, 4);
    }

    #[test]
    fn full_1a() {
        let out = Program::new("3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8", "8")
            .run_prog()
            .output;
        assert_eq!(out, Some(1));
    }

    #[test]
    fn full_1b() {
        let out = Program::new("3, 3, 1107, -1, 8, 3, 4, 3, 99", "11")
            .run_prog()
            .output;
        assert_eq!(out, Some(0));
    }
    #[test]
    fn full_1c() {
        let out = Program::new("3, 3, 1107, -1, 8, 3, 4, 3, 99", "2")
            .run_prog()
            .output;
        assert_eq!(out, Some(1));
    }
}
