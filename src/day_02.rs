fn get_output(noun: usize, verb: usize) -> usize {
    let mut input = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 13, 19, 23, 2, 23, 9, 27,
        1, 6, 27, 31, 2, 10, 31, 35, 1, 6, 35, 39, 2, 9, 39, 43, 1, 5, 43, 47, 2, 47, 13, 51, 2,
        51, 10, 55, 1, 55, 5, 59, 1, 59, 9, 63, 1, 63, 9, 67, 2, 6, 67, 71, 1, 5, 71, 75, 1, 75, 6,
        79, 1, 6, 79, 83, 1, 83, 9, 87, 2, 87, 10, 91, 2, 91, 10, 95, 1, 95, 5, 99, 1, 99, 13, 103,
        2, 103, 9, 107, 1, 6, 107, 111, 1, 111, 5, 115, 1, 115, 2, 119, 1, 5, 119, 0, 99, 2, 0, 14,
        0,
    ];
    input[1] = noun;
    input[2] = verb;
    let mut index = 0;
    let mut done = false;
    while !done {
        if input[index] == 99 {
            done = true
        } else {
            let op = input[index];
            let op1 = input[input[index + 1]];
            let op2 = input[input[index + 2]];
            let result;
            if op == 1 {
                result = op1 + op2;
            } else {
                result = op1 * op2;
            };
            input[input[index + 3]] = result;
            index += 4
        }
    }
    return input[0];
}

pub fn day2a() {
    println!("2a: {}", get_output(12, 2));
}

pub fn day2b() {
    let mut done = false;
    let mut noun = 0;
    let mut verb = 0;
    while !done {
        if get_output(noun, verb) == 19690720 {
            println!("2b: {}", 100 * noun + verb);
            done = true
        }
        noun += 1;
        if noun == 100 {
            verb += 1;
            noun = 0
        }
        if verb == 100 {
            println!("Failed!!");
            done = true
        }
    }
}
