#[derive(Clone)]
pub struct Program {
    code: Vec<i128>,
    inst_ptr: usize,
    done: bool,
    input: Vec<i128>,
    input_ptr: usize,
    output: Option<i128>,
    relative_base: usize,
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

    fn new_from_vecs(arr: &Vec<i128>, input: &Vec<i128>) -> Program {
        let mut prog = Program {
            code: arr.clone(),
            inst_ptr: 0,
            done: false,
            input: input.clone(),
            input_ptr: 0,
            output: None,
            relative_base: 0,
        };
        prog.code.resize(10000, 0);
        prog
    }

    pub fn push_input(&mut self, input: i128) {
        self.input.push(input)
    }

    pub fn get_output(&mut self) -> i128 {
        let output = self.output;
        self.output = None;
        output.unwrap()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    fn get_opcode(&self) -> i128 {
        let instruction = self.code[self.inst_ptr];
        let opcode = instruction % 100;
        if (opcode < 0 || opcode > 9) && opcode != 99 {
            panic!("Invalid opcode {}", opcode);
        }
        instruction % 100
    }

    fn get_operand_mode(&self, operand_index: usize) -> usize {
        let instruction = self.code[self.inst_ptr];
        let operand_mode;
        match operand_index {
            1 => operand_mode = (instruction / 100) % 10,
            2 => operand_mode = (instruction / 1000) % 10,
            3 => operand_mode = (instruction / 10000) % 10,
            _ => panic!("Invalid operand_index {}", operand_index),
        }
        if operand_mode < 0 || operand_mode > 2 {
            panic!("Invalid operand mode {}", operand_mode);
        }
        operand_mode as usize
    }

    fn get_operand_addr(&self, operand_index: usize) -> i128 {
        let op_mode = self.get_operand_mode(operand_index);
        let mut op_addr = self.code[self.inst_ptr + operand_index];
        if op_mode == 2 {
            op_addr = (self.relative_base as i128 + op_addr) as i128
        }
        op_addr
    }

    fn get_operand(&self, operand_index: usize) -> i128 {
        let op_mode = self.get_operand_mode(operand_index);
        let op_addr = self.get_operand_addr(operand_index);
        if op_mode == 1 {
            return op_addr;
        } else {
            return self.code[op_addr as usize];
        }
    }

    fn complete_instruction(&self) -> String {
        let opcode = self.get_opcode();
        match opcode {
            1 | 2 | 7 | 8 => {
                return format!(
                    "{} {} {} {}",
                    self.code[self.inst_ptr],
                    self.code[self.inst_ptr + 1],
                    self.code[self.inst_ptr + 2],
                    self.code[self.inst_ptr + 3],
                )
            }
            5 | 6 => {
                return format!(
                    "{} {} {}",
                    self.code[self.inst_ptr],
                    self.code[self.inst_ptr + 1],
                    self.code[self.inst_ptr + 2]
                )
            }
            3 | 4 | 9 => {
                return format!(
                    "{} {}",
                    self.code[self.inst_ptr],
                    self.code[self.inst_ptr + 1]
                )
            }
            99 => return format!("{}", self.code[self.inst_ptr]),

            _ => return "huh???".to_string(),
        }
    }

    fn step_prog(&self) -> Program {
        let mut prog = self.clone();
        prog.done = false;
        let opcode = self.get_opcode();

        print!("----  op {} : {} : ", opcode, prog.complete_instruction());
        match opcode {
            1 => {
                let op1 = prog.get_operand(1);
                let op2 = prog.get_operand(2);
                let res_addr = prog.get_operand_addr(3) as usize;
                prog.code[res_addr] = op1 + op2;
                prog.inst_ptr += 4;
                print!("{} + {} ({}) => addr {}", op1, op2, op1 + op2, res_addr);
            }
            2 => {
                let op1 = prog.get_operand(1);
                let op2 = prog.get_operand(2);
                let res_addr = prog.get_operand_addr(3) as usize;
                prog.code[res_addr] = op1 * op2;
                prog.inst_ptr += 4;
                print!("{} + {} ({}) => addr {}", op1, op2, op1 * op2, res_addr);
            }
            3 => {
                let res_addr = prog.get_operand_addr(1) as usize;
                let input = prog.input[prog.input_ptr];
                print!("Store input {} => addr {}", input, res_addr);
                prog.code[res_addr] = input;
                prog.inst_ptr += 2;
                prog.input_ptr += 1;
            }
            4 => {
                let op1 = prog.get_operand(1);
                print!("output {}", op1);
                prog.output = Some(op1);
                prog.inst_ptr += 2;
            }
            5 => {
                let op1 = prog.get_operand(1);
                let op2 = prog.get_operand(2);

                if op1 != 0 {
                    print!("{} != 0 so set inst_ptr to {}", op1, op2);
                    prog.inst_ptr = op2 as usize;
                } else {
                    print!("{} == 0 so add 3 to inst_ptr", op1);
                    prog.inst_ptr += 3;
                }
            }
            6 => {
                let op1 = prog.get_operand(1);
                let op2 = prog.get_operand(2);
                if op1 == 0 {
                    prog.inst_ptr = op2 as usize;
                } else {
                    prog.inst_ptr += 3;
                }
            }
            7 => {
                let op1 = prog.get_operand(1);
                let op2 = prog.get_operand(2);
                let res_addr = prog.get_operand_addr(3) as usize;
                if op1 < op2 {
                    print!("{} < {} so store 1 in addr {}", op1, op2, res_addr);
                    prog.code[res_addr] = 1;
                } else {
                    print!("{} >= {} so store 0 in addr {}", op1, op2, res_addr);
                    prog.code[res_addr] = 0;
                }
                prog.inst_ptr += 4;
            }
            8 => {
                let op1 = prog.get_operand(1);
                let op2 = prog.get_operand(2);
                let res_addr = prog.get_operand_addr(3) as usize;
                prog.code[res_addr] = if op1 == op2 { 1 } else { 0 };
                prog.inst_ptr += 4;
            }
            9 => {
                let op1 = prog.get_operand(1);
                let new_relative_base = prog.relative_base as i128 + op1;
                print!("set relative base to {}", new_relative_base);
                prog.relative_base = new_relative_base as usize;
                prog.inst_ptr += 2;
            }
            99 => {
                prog.done = true;
            }
            _ => {
                prog.done = true;
            }
        }
        println!(" ");
        return prog;
    }

    pub fn run_prog(&self) -> Program {
        let mut prog = self.clone();
        loop {
            if prog.is_done() || prog.output.is_some() {
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
        assert_eq!(prog.is_done(), false);
        assert_eq!(prog.code[3], 70);
    }

    #[test]
    fn run_prog_1() {
        let prog = Program::new("1, 1, 1, 4, 99, 5, 6, 0, 99", "0");
        let prog = prog.run_prog();
        assert_eq!(prog.is_done(), true);
        assert_eq!(prog.code[0], 30);
    }

    #[test]
    fn run_prog_2() {
        let prog = Program::new("1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50", "0");
        let prog = prog.run_prog();
        assert_eq!(prog.is_done(), true);
        assert_eq!(prog.code[0], 3500);
    }

    #[test]
    fn run_prog_input() {
        let prog = Program::new("3, 3, 99, 11", "666");
        let prog = prog.run_prog();
        assert_eq!(prog.is_done(), true);
        assert_eq!(prog.code[3], 666);
    }

    #[test]
    fn run_prog_2_inputs() {
        let prog = Program::new("3, 9, 3, 10, 1, 9, 10, 0, 99, 99, 99, 99, 99", "222, 111");
        let prog = prog.run_prog();
        assert_eq!(prog.is_done(), true);
        assert_eq!(prog.code[0], 333);
    }

    #[test]
    fn push_input() {
        let mut prog = Program::new("3, 3, 99, 11", "");
        prog.push_input(666);
        prog = prog.run_prog();
        assert_eq!(prog.is_done(), true);
        assert_eq!(prog.code[3], 666);
    }

    #[test]
    fn run_prog_output() {
        let mut prog = Program::new("3, 0, 4, 0, 99", "666");
        prog = prog.run_prog();
        assert_eq!(prog.get_output(), 666);
    }

    #[test]
    fn run_prog_modes() {
        let prog = Program::new("1002, 4, 3, 4, 33", "666");
        let prog = prog.run_prog();
        assert_eq!(prog.is_done(), true);
        assert_eq!(prog.inst_ptr, 4);
    }

    #[test]
    fn full_1a() {
        let out = Program::new("3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8", "8")
            .run_prog()
            .get_output();
        assert_eq!(out, 1);
    }

    #[test]
    fn full_1b() {
        let out = Program::new("3, 3, 1107, -1, 8, 3, 4, 3, 99", "11")
            .run_prog()
            .get_output();
        assert_eq!(out, 0);
    }
    #[test]
    fn full_1c() {
        let out = Program::new("3, 3, 1107, -1, 8, 3, 4, 3, 99", "2")
            .run_prog()
            .get_output();
        assert_eq!(out, 1);
    }

    #[test]
    fn opcode_9_a() {
        let code = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let code_vec: Vec<i128> = code
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let mut prog = Program::new(code, "0");
        let mut index = 0;
        loop {
            if prog.is_done() {
                break;
            }
            prog = prog.run_prog();
            let output = prog.get_output();
            println!("{}", output);
            if output == 99 {
                break;
            };
            assert_eq!(output, code_vec[index]);
            index += 1;
        }
    }

    #[test]
    fn opcode_9_b() {
        let code = "1102,34915192,34915192,7,4,7,99,0";
        let output = Program::new(code, "0").run_prog().get_output();
        assert_eq!(format!("{}", output).len(), 16);
    }

    #[test]
    fn opcode_9_c() {
        let code = "104,1125899906842624,99";
        let output = Program::new(code, "0").run_prog().get_output();
        assert_eq!(output, 1125899906842624);
    }

    #[test]
    fn test_203() {
        let mut prog = Program::new("203,50,99", "11");
        prog.relative_base = 100;
        prog = prog.run_prog();
        assert_eq!(prog.code[150], 11);
    }
}
