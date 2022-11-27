use crate::ContinuousRandomVariableGenerator;
use crate::RandomNumberGenerator;
use std::collections::HashMap;

pub struct LawOfLargeNumbersSimulator<T>
where
    T: Fn(f64) -> f64,
{
    inverse_pdf: T,
    sample_sizes: Vec<i32>,
    num_estimates: i32,
}

impl<T> LawOfLargeNumbersSimulator<T>
where
    T: Fn(f64) -> f64,
{
    pub fn new(
        inverse_pdf: T,
        sample_sizes: Vec<i32>,
        num_estimates: i32,
    ) -> LawOfLargeNumbersSimulator<T> {
        LawOfLargeNumbersSimulator {
            inverse_pdf,
            sample_sizes,
            num_estimates,
        }
    }

    pub fn simulate(
        &mut self,
        random_number_generator: &mut RandomNumberGenerator,
    ) -> HashMap<i32, Vec<f64>> {
        let mut results: HashMap<i32, Vec<f64>> = HashMap::new();
        let mut random_variable =
            ContinuousRandomVariableGenerator::new(random_number_generator, &mut self.inverse_pdf);
        for sample_size in &self.sample_sizes {
            for _ in 0..self.num_estimates {
                let mut sample_mean = 0f64;
                for _ in 0..*sample_size {
                    sample_mean += random_variable.generate_realization();
                }
                sample_mean /= *sample_size as f64;
                results
                    .entry(*sample_size)
                    .or_insert(Vec::new())
                    .push(sample_mean);
            }
        }
        results
    }

    pub fn sample_mean(
        &mut self,
        random_number_generator: &mut RandomNumberGenerator,
        n: i32,
    ) -> f64 {
        let mut sample_sum = 0f64;
        let mut random_variable =
            ContinuousRandomVariableGenerator::new(random_number_generator, &mut self.inverse_pdf);
        for _ in 0..n {
            sample_sum += random_variable.generate_realization();
        }
        sample_sum / n as f64
    }
}
