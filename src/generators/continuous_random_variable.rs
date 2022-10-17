use crate::generators::random_number::RandomNumberGenerator;

pub struct ContinuousRandomVariableGenerator<'a, T>
where
    T: Fn(f64) -> f64,
{
    rng: &'a mut RandomNumberGenerator,
    f_inverse: &'a mut T,
}

impl<T> ContinuousRandomVariableGenerator<'_, T>
where
    T: Fn(f64) -> f64,
{
    pub fn new<'a>(
        rng: &'a mut RandomNumberGenerator,
        f_inverse: &'a mut T,
    ) -> ContinuousRandomVariableGenerator<'a, T> {
        ContinuousRandomVariableGenerator { rng, f_inverse }
    }

    pub fn generate_realization(&mut self) -> f64 {
        let u_i = self.rng.get_next_number();
        let x_i = (self.f_inverse)(u_i);
        x_i
    }
}
