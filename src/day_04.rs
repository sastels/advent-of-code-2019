fn check_number_a(n: usize) -> bool {
    let n_str = n.to_string();
    let mut has_double = false;
    let mut prev_char = '/';
    for c in n_str.chars() {
        if c < prev_char {
            return false;
        } else if c == prev_char {
            has_double = true;
        }
        prev_char = c
    }
    return has_double;
}

// looking at four chars in a row a,b,c,d
// want a != b, b==c and c != d
fn check_number_b(n: usize) -> bool {
    let n_str = format!("{}{}", n, ";");
    let mut has_double = false;
    let mut a = '/';
    let mut b = '.';
    let mut c = '+';
    for d in n_str.chars() {
        if d < c {
            return false;
        } else if (a != b) && (b == c) && (c != d) {
            has_double = true;
        }
        a = b;
        b = c;
        c = d;
    }
    return has_double;
}

pub fn day4a() {
    let mut num_matches_a = 0;
    let mut num_matches_b = 0;
    for n in 125730..579381 {
        if check_number_a(n) {
            num_matches_a += 1;
            if check_number_b(n) {
                num_matches_b += 1;
            }
        }
    }
    println!("4a: {}", num_matches_a);
    println!("4b: {}", num_matches_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_number_a() {
        assert_eq!(check_number_a(111111), true);
        assert_eq!(check_number_a(223450), false);
        assert_eq!(check_number_a(123789), false);
        assert_eq!(check_number_a(1233789), true);
    }

    #[test]
    fn test_check_number_b() {
        assert_eq!(check_number_b(112233), true);
        assert_eq!(check_number_b(123444), false);
        assert_eq!(check_number_b(111122), true);
        assert_eq!(check_number_b(1233789), true);
        assert_eq!(check_number_b(12337289), false);
    }
}
