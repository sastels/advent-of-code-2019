fn fuel_step(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn day1a() {
    let mut weights = [
        60077, 148191, 77039, 138122, 109889, 140187, 116411, 94170, 149976, 70913, 64860, 149414,
        146246, 143889, 105615, 143060, 82445, 108411, 54193, 102892, 136407, 147470, 129652,
        75887, 104208, 131104, 82815, 72880, 104513, 64130, 112780, 71099, 108700, 137290, 53878,
        146277, 68897, 118713, 141895, 74593, 111125, 62545, 114375, 140815, 137314, 129250, 78915,
        63391, 133745, 99785, 104597, 140050, 118183, 67394, 84529, 82530, 109998, 126649, 56545,
        139507, 71148, 71421, 99495, 59804, 118055, 110401, 84557, 69630, 130090, 110550, 115260,
        88256, 97149, 62192, 142398, 85462, 132228, 102567, 67005, 54685, 83284, 142337, 117615,
        90445, 78048, 68070, 99049, 83965, 124845, 76143, 96194, 92299, 128558, 64150, 85126,
        63567, 103223, 125883, 58363, 72622,
    ];

    for x in weights.iter_mut() {
        *x = fuel_step(*x)
    }
    let result: i32 = weights.iter().sum();
    println!("day 1a: {}", result); // 3371958
}

fn total_fuel(mass: i32) -> i32 {
    let fuel = fuel_step(mass);
    if fuel <= 0 {
        return 0;
    } else {
        return fuel + total_fuel(fuel);
    }
}

pub fn day1b() {
    let mut weights = [
        60077, 148191, 77039, 138122, 109889, 140187, 116411, 94170, 149976, 70913, 64860, 149414,
        146246, 143889, 105615, 143060, 82445, 108411, 54193, 102892, 136407, 147470, 129652,
        75887, 104208, 131104, 82815, 72880, 104513, 64130, 112780, 71099, 108700, 137290, 53878,
        146277, 68897, 118713, 141895, 74593, 111125, 62545, 114375, 140815, 137314, 129250, 78915,
        63391, 133745, 99785, 104597, 140050, 118183, 67394, 84529, 82530, 109998, 126649, 56545,
        139507, 71148, 71421, 99495, 59804, 118055, 110401, 84557, 69630, 130090, 110550, 115260,
        88256, 97149, 62192, 142398, 85462, 132228, 102567, 67005, 54685, 83284, 142337, 117615,
        90445, 78048, 68070, 99049, 83965, 124845, 76143, 96194, 92299, 128558, 64150, 85126,
        63567, 103223, 125883, 58363, 72622,
    ];

    for x in weights.iter_mut() {
        *x = total_fuel(*x)
    }

    let result: i32 = weights.iter().sum();
    println!("day 1b: {}", result); // 5055050
}
