use crate::generators::continuous_random_variable::ContinuousRandomVariableGenerator;
use crate::generators::discrete_random_variable::DiscreteRandomVariableGenerator;
use crate::generators::random_number::RandomNumberGenerator;

pub struct CallingProcess<T>
where
    T: Fn(f64) -> f64,
{
    w: f64,
    x_pdf_inverse: T,
    // call if available
    t_d: f32, // time it takes to dial a number
    t_b: f32, // time it takes to detect a busy signal
    t_u: f32, // time it takes to wait for 5 rings
    t_e: f32, // time it takes to end the call
    p_b: f32, // the probability the line is busy
    p_u: f32, // the probability the user is unavailable
    p_a: f32, // the probability the user is available within X seconds
}

impl<T> CallingProcess<T>
where
    T: Fn(f64) -> f64,
{
    pub fn new<'a>(
        x_pdf_inverse: T,
        t_d: f32,
        t_b: f32,
        t_u: f32,
        t_e: f32,
        p_b: f32,
        p_u: f32,
        p_a: f32,
    ) -> CallingProcess<T> {
        CallingProcess {
            w: 0.0,
            x_pdf_inverse,
            t_d,
            t_b,
            t_u,
            t_e,
            p_b,
            p_u,
            p_a,
        }
    }

    pub fn simulate(&mut self, random_number_generator: &mut RandomNumberGenerator, verbose: bool) {
        let mut num_calls = 0;
        while num_calls < 4 {
            // Make the Call
            num_calls += 1;
            if verbose {
                println!("making call number {}", num_calls);
            }
            self.w += self.t_d as f64;
            if verbose {
                println!("Dialed call number {} after: {}s", num_calls, self.w);
            }
            // See what the status of the call is
            // 1 -> busy
            // 2 -> unavailable
            // 3 -> available
            let caller_status_realization = {
                let caller_cdf = |x: f64| -> f64 {
                    if x == 1.0 {
                        self.p_b as f64
                    } else if x == 2.0 {
                        (self.p_u + self.p_b) as f64
                    } else if x == 3.0 {
                        (self.p_a + self.p_u + self.p_b) as f64
                    } else {
                        0.0
                    }
                };
                let sample_space = vec![1.0, 2.0, 3.0];
                let mut caller_status = DiscreteRandomVariableGenerator::new(
                    random_number_generator,
                    caller_cdf,
                    sample_space,
                );
                caller_status.generate_realization()
            };

            let mut x = ContinuousRandomVariableGenerator::new(
                random_number_generator,
                &mut self.x_pdf_inverse,
            );

            if caller_status_realization == 1.0 {
                // The caller is busy
                if verbose {
                    println!("On call number {} the user was busy", num_calls);
                }
                self.w += self.t_b as f64;
                if verbose {
                    println!("Call number {} busy after {}s", num_calls, self.w);
                }
            } else if caller_status_realization == 2.0 {
                // The caller is unavailable
                if verbose {
                    println!("On call number {} the user was unavailable", num_calls);
                }
                self.w += self.t_u as f64;
                if verbose {
                    println!("Call number {} unavailable after {}s", num_calls, self.w);
                }
            } else if caller_status_realization == 3.0 {
                // The caller is available
                if verbose {
                    println!("On call number {} the user was available", num_calls);
                }
                let wait_time = x.generate_realization();
                if verbose {
                    println!("Caller picked up after {}s", wait_time);
                }
                if wait_time <= self.t_u as f64 {
                    self.w += wait_time as f64;
                    if verbose {
                        println!("Call number {} available after {}s", num_calls, self.w);
                    }
                    break;
                } else {
                    self.w += self.t_u as f64;
                    if verbose {
                        println!("Call number {} unavailable after {}s", num_calls, self.w);
                    }
                }
            }

            self.w += self.t_e as f64;
            if verbose {
                println!("Call number {} hung up after {}s", num_calls, self.w);
            }
        }
    }

    pub fn get_total_time(self) -> f64 {
        self.w
    }
}
