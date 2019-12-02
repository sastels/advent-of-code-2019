fn fuel_step(mass: i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel(mass: i32) -> i32 {
    let fuel = fuel_step(mass);
    if fuel <= 0 {
        return 0;
    } else {
        return fuel + total_fuel(fuel);
    }
}

pub fn day1a(weights: &mut [i32]) {
    for x in weights.iter_mut() {
        *x = fuel_step(*x)
    }
    let result: i32 = weights.iter().sum();
    println!("day 1a: {}", result); // 3371958
}

pub fn day1b(weights: &mut [i32]) {
    for x in weights.iter_mut() {
        *x = total_fuel(*x)
    }

    let result: i32 = weights.iter().sum();
    println!("day 1b: {}", result); // 5055050
}
