use miniproject1::{self, RandomNumberGenerator};
fn main() {
    let k = pow(2, 17);
    let mut rng = RandomNumberGenerator::new(1000, 24693, 3517, k);
    let u1 = rng.get_next_number();
    let u2 = rng.get_next_number();
    let u3 = rng.get_next_number();

    println!("u1: {} u2: {}, u3: {}", u1, u2, u3);
}

fn pow(base: i64, power: i64) -> i64 {
    let mut val = 1;
    for _ in 0..power {
        val *= base;
    }
    return val;
}
