use crate::generators::random_number::RandomNumberGenerator;

pub struct DiscreteRandomVariableGenerator<'a, T>
where
    T: Fn(f64) -> f64,
{
    rng: &'a mut RandomNumberGenerator,
    pmf: T,
    sample_space: Vec<f64>,
}

impl<T> DiscreteRandomVariableGenerator<'_, T>
where
    T: Fn(f64) -> f64,
{
    pub fn new(
        rng: &mut RandomNumberGenerator,
        pmf: T,
        sample_space: Vec<f64>,
    ) -> DiscreteRandomVariableGenerator<T> {
        DiscreteRandomVariableGenerator {
            rng,
            pmf,
            sample_space,
        }
    }

    pub fn generate_realization(&mut self) -> f64 {
        let u_i = self.rng.get_next_number();
        for i in &mut self.sample_space {
            if (self.pmf)(*i) > u_i {
                return *i;
            }
        }
        return -1.0;
    }
}
