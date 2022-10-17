mod generators;
use generators::continuousRandomVariable::ContinuousRandomVariableGenerator;
use generators::discreteRandomVariable::DiscreteRandomVariableGenerator;
use generators::randomNumber::RandomNumberGenerator;

pub struct CallingProcess<T>
where
    T: Fn(f64) -> f64,
{
    w: f64,
    x: ContinuousRandomVariableGenerator<T>,
    t_d: f32,
    t_b: f32,
    t_u: f32,
    t_e: f32,
    p_b: f32,
    p_u: f32,
    p_a: f32,
}

impl<T> CallingProcess<T>
where
    T: Fn(f64) -> f64,
{
    fn new(
        x: ContinuousRandomVariableGenerator<T>,
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
            x,
            t_d,
            t_b,
            t_u,
            t_e,
            p_b,
            p_u,
            p_a,
        }
    }

    fn simulate() -> f32 {
        0.0
    }
}

pub fn run() {
    println!("Starting simulation");
    let mut rng = RandomNumberGenerator::new(1000, 24693, 3517, i64::pow(2, 17));
    let mut random_numbers: Vec<f64> = Vec::new();

    for _ in 0..53 {
        random_numbers.push(rng.get_next_number());
    }

    println!(
        "The first three random numbers are: \n u1: {} \n u2: {} \n u3: {}",
        random_numbers[0], random_numbers[1], random_numbers[2]
    );
    println!(
        "u51, u52, and u53 are: \n u51: {}, \n u52: {}, \n u53: {}",
        random_numbers[50], random_numbers[51], random_numbers[52]
    );
}

pub fn run_calling_process() -> f64 {
    0.0
}

pub fn test_discrete_random_variable() {
    let rng = RandomNumberGenerator::new(1000, 24693, 3517, i64::pow(2, 17));
    let pdf = |x: f64| -> f64 {
        if x == 1.0 {
            0.1
        } else if x == 2.0 {
            0.3
        } else if x == 3.0 {
            0.7
        } else if x == 4.0 {
            1.0
        } else {
            0.0
        }
    };
    let sample_space = vec![1.0, 2.0, 3.0, 4.0];
    let mut random_variable_x = DiscreteRandomVariableGenerator::new(rng, pdf, sample_space);
    let count = 10000;
    let mut reals = vec![0, 0, 0, 0];
    for _ in 0..count {
        reals[(random_variable_x.generate_realization() - 1.0) as usize] += 1;
    }
    for i in 0..reals.len() {
        println!("PMF of {}: {}", i + 1, reals[i] as f32 / count as f32);
    }
}
