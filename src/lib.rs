mod calling_process;
mod generators;

use generators::discrete_random_variable::DiscreteRandomVariableGenerator;
use generators::random_number::RandomNumberGenerator;
use std::time::UNIX_EPOCH;

use std::cmp::Ordering;
use std::time::SystemTime;

use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    println!("Starting simulation");
    test_random_number_generator();

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong")
        .as_millis()
        % 1000;
    let mut simulation_rng = RandomNumberGenerator::new(seed as i64, 24693, 3517, i64::pow(2, 17));
    let mut calling_realizations: Vec<f64> = Vec::new();
    for _ in 1..50000 {
        let waiting_time = run_calling_process(&mut simulation_rng);
        calling_realizations.push(waiting_time);
    }
    estimate_quantities(&mut calling_realizations);
    let saved = save_results(&mut calling_realizations, "results.csv");
}

pub fn run_calling_process(random_number_generator: &mut RandomNumberGenerator) -> f64 {
    let waiting_time_pdf_inverse = |u: f64| -> f64 { -12.0 * (1.0 - u).ln() };
    let mut calling_process = calling_process::CallingProcess::new(
        waiting_time_pdf_inverse,
        6.0,
        3.0,
        25.0,
        1.0,
        0.2,
        0.3,
        0.5,
    );
    calling_process.simulate(random_number_generator, false);
    calling_process.get_total_time()
}

pub fn estimate_quantities(results: &mut Vec<f64>) {
    results.sort_by(|a, b| {
        if a > b {
            Ordering::Greater
        } else if a < b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    // Mean
    let mean: f64 = results.iter().sum::<f64>() / results.len() as f64;
    println!("The estimated mean of the results is: {}", mean);
    // First Quartile
    let first_quartile = results[(results.len() / 4) as usize];
    // Median
    let median = results[(results.len() / 2) as usize];
    // Third Quartile
    let third_quartile = results[(3 * results.len() / 4) as usize];
    println!(
        "First Quartile: {}, Median: {}, Third Quartile: {}",
        first_quartile, median, third_quartile
    );
}

pub fn save_results(results: &mut Vec<f64>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for result in results {
        file.write(format!("{} \n", result).as_bytes())?;
    }
    Ok(())
}

pub fn test_random_number_generator() {
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

pub fn test_discrete_random_variable() {
    let mut rng = RandomNumberGenerator::new(1000, 24693, 3517, i64::pow(2, 17));
    let pmf = |x: f64| -> f64 {
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
    let mut random_variable_x = DiscreteRandomVariableGenerator::new(&mut rng, pmf, sample_space);
    let count = 10000;
    let mut reals = vec![0, 0, 0, 0];
    for _ in 0..count {
        reals[(random_variable_x.generate_realization() - 1.0) as usize] += 1;
    }
    for i in 0..reals.len() {
        println!("PMF of {}: {}", i + 1, reals[i] as f32 / count as f32);
    }
}
