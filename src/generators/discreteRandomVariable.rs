use crate::generators::randomNumber::RandomNumberGenerator;

pub struct DiscreteRandomVariableGenerator<T>
where
    T: Fn(f64) -> f64,
{
    rng: RandomNumberGenerator,
    pdf: T,
    sample_space: Vec<f64>,
}

impl<T> DiscreteRandomVariableGenerator<T>
where
    T: Fn(f64) -> f64,
{
    pub fn new(
        rng: RandomNumberGenerator,
        pdf: T,
        sample_space: Vec<f64>,
    ) -> DiscreteRandomVariableGenerator<T> {
        DiscreteRandomVariableGenerator {
            rng,
            pdf,
            sample_space,
        }
    }

    pub fn generate_realization(&mut self) -> f64 {
        let u_i = self.rng.get_next_number();
        for i in &mut self.sample_space {
            if (self.pdf)(*i) > u_i {
                return *i;
            }
        }
        return -1.0;
    }
}
