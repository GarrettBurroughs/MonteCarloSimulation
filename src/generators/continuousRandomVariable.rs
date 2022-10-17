use crate::generators::randomNumber::RandomNumberGenerator;

pub struct ContinuousRandomVariableGenerator<T>
where
    T: Fn(f64) -> f64,
{
    rng: RandomNumberGenerator,
    f_inverse: T,
}

impl<T> ContinuousRandomVariableGenerator<T>
where
    T: Fn(f64) -> f64,
{
    pub fn new(rng: RandomNumberGenerator, f_inverse: T) -> ContinuousRandomVariableGenerator<T> {
        ContinuousRandomVariableGenerator { rng, f_inverse }
    }

    pub fn generate_realization(&mut self) -> f64 {
        let u_i = self.rng.get_next_number();
        let x_i = (self.f_inverse)(u_i);
        x_i
    }
}
