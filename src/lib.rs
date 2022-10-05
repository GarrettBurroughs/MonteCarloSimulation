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
