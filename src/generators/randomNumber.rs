pub struct RandomNumberGenerator {
    current: i64,    // x_i
    multiplier: i64, // a
    increment: i64,  // c
    modulus: i64,    // K
}

impl RandomNumberGenerator {
    pub fn new(
        starting_value: i64,
        multiplier: i64,
        increment: i64,
        modulus: i64,
    ) -> RandomNumberGenerator {
        RandomNumberGenerator {
            current: starting_value,
            multiplier,
            increment,
            modulus,
        }
    }

    // x_i = (a * x_(i-1) + c) % k
    pub fn get_next_number(&mut self) -> f64 {
        let x_i = (self.multiplier * self.current + self.increment) % self.modulus;
        self.current = x_i;
        x_i as f64 / self.modulus as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_three_numbers() {
        let mut rng = RandomNumberGenerator::new(1000, 24693, 3517, i64::pow(2, 17));
        assert_eq!(true, within_delta(0.4195, rng.get_next_number()));
        assert_eq!(true, within_delta(0.0425, rng.get_next_number()));
        assert_eq!(true, within_delta(0.1274, rng.get_next_number()));
    }

    fn within_delta(a: f64, b: f64) -> bool {
        let delta = 0.0001;
        f64::abs(a - b) < delta
    }
}
