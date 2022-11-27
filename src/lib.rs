mod calling_process;
mod generators;
mod large_numbers_sim;

use generators::continuous_random_variable::ContinuousRandomVariableGenerator;
use generators::discrete_random_variable::DiscreteRandomVariableGenerator;
use generators::random_number::RandomNumberGenerator;
use large_numbers_sim::LawOfLargeNumbersSimulator;
use std::collections::HashMap;
use std::env;

use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn mini_project_1() {
    println!("Starting simulation");
    test_random_number_generator();
    let mut simulation_rng = RandomNumberGenerator::new_default();
    let mut calling_realizations: Vec<f64> = Vec::new();
    for _ in 0..1000 {
        let waiting_time = run_calling_process(&mut simulation_rng);
        calling_realizations.push(waiting_time);
    }
    estimate_quantities(&mut calling_realizations);
    let saved = save_vector_results(&mut calling_realizations, "results.csv");

    if let Err(save_error) = saved {
        eprintln!("{}", save_error);
    }
}

pub fn mini_project_2() {
    let t = 57f64;
    let a = 1f64 / t;
    let inverse_rayleigh_cdf = |u: f64| -> f64 { f64::sqrt((-2f64 * (1f64 - u).ln()) / (a * a)) };
    let mut sim = LawOfLargeNumbersSimulator::new(
        inverse_rayleigh_cdf,
        vec![10, 30, 50, 100, 250, 500, 1000],
        110,
    );
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong")
        .as_millis()
        % 1000;
    let true_random = env::var("TRUE_RANDOM").is_ok();
    let mut rng = RandomNumberGenerator::new(
        if true_random { 1000 } else { seed as i64 },
        24693,
        3967,
        i64::pow(2, 18),
    );

    let results = sim.simulate(&mut rng);
    let filename = "coordinates.dat";
    let res = save_coordinate_results(results, filename);
    if let Err(_) = res {
        eprintln!("Error saving coordinate file");
    }
    println!("Saved law of large numbers simulation to {}", filename);

    let samples = vec![(3, 5), (9, 25), (27, 110), (81, 550)];
    let mut m: HashMap<i32, Vec<f64>> = HashMap::new();
    for (n, k) in &samples {
        m.insert(*n, Vec::new());
        for _ in 0..*k {
            let value = sim.sample_mean(&mut rng, *n as i32);
            m.get_mut(n).unwrap().push(value);
        }
    }
    let mut estimated_means: HashMap<i32, f64> = HashMap::new();
    let mut estimated_variances: HashMap<i32, f64> = HashMap::new();

    for (n, k) in &samples {
        let mut sum: f64 = 0f64;
        for i in 0..*k {
            sum += m[n][i];
        }
        estimated_means.insert(*n, sum / *k as f64);
    }

    for (n, k) in &samples {
        let mut var: f64 = 0f64;
        for i in 0..*k {
            var += f64::powi(m[n][i], 2) - f64::powi(estimated_means[n], 2);
        }
        estimated_variances.insert(*n, var / *k as f64);
    }

    println!(
        "Sample Means: {:?} \n Sample Variances: {:?}",
        estimated_means, estimated_variances
    );

    let mut standard_vars: HashMap<i32, Vec<f64>> = HashMap::new();
    for (n, k) in &samples {
        standard_vars.insert(*n, Vec::new());
        for i in 0..*k {
            let standardized = (m[n][i] - estimated_means[n]) / f64::sqrt(estimated_variances[n]);
            standard_vars.get_mut(n).unwrap().push(standardized);
        }
    }
    // println!("Standardized Variables: {:?}", standard_vars);
    let z_set = vec![-1.4, -1.0, -0.5, 0.5, 1.0, 1.4];
    let mut distributions: HashMap<i32, Vec<f64>> = HashMap::new();
    for (n, k) in &samples {
        distributions.insert(*n, Vec::new());
        for z in &z_set {
            let mut cdf = 0f64;
            for value in &standard_vars[n] {
                if value <= z {
                    cdf += 1 as f64;
                }
            }
            cdf /= standard_vars[n].len() as f64;
            distributions.get_mut(n).unwrap().push(cdf);
        }
    }
    println!("CDFS {:?}", distributions);
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

fn estimate_quantities(results: &mut Vec<f64>) {
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
    let p_less_15 = get_cdf(results, 15.0);
    let p_less_20 = get_cdf(results, 20.0);
    let p_less_30 = get_cdf(results, 30.0);
    let p_greater_40 = 1.0 - get_cdf(results, 40.0);
    let p_greater_w5 = 1.0 - get_cdf(results, 75.0);
    let p_greater_w6 = 1.0 - get_cdf(results, 100.0);
    let p_greater_w7 = 1.0 - get_cdf(results, 125.0);
    println!(
        "W <= 15: {}, W <= 20: {}, W <= 30: {}, W > 40: {}, W > 75: {}, W > 100: {}, W > 125: {}",
        p_less_15, p_less_20, p_less_30, p_greater_40, p_greater_w5, p_greater_w6, p_greater_w7
    );
}

fn get_cdf(results: &Vec<f64>, value: f64) -> f32 {
    let mut total_events = 0;
    for result in results {
        if *result <= value {
            total_events += 1;
        }
    }
    total_events as f32 / results.len() as f32
}

fn save_vector_results(results: &Vec<f64>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for result in results {
        file.write(format!("{}, \n", result).as_bytes())?;
    }
    Ok(())
}

fn save_coordinate_results(results: HashMap<i32, Vec<f64>>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write(format!("sample_size sample_mean \n").as_bytes())?;
    for key in results.keys() {
        results.get(key).unwrap().iter().for_each(|x| {
            let res = file.write(format!("{} {} \n", key, x).as_bytes());
            if let Err(e) = res {
                eprintln!("Error writing to file");
            }
        });
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
    let pmf = |y: f64| -> f64 {
        if y == 1.0 {
            0.1
        } else if y == 2.0 {
            0.2
        } else if y == 3.0 {
            0.4
        } else if y == 4.0 {
            0.3
        } else {
            0.0
        }
    };
    let sample_space = vec![1.0, 2.0, 3.0, 4.0];
    let mut random_variable_y = DiscreteRandomVariableGenerator::new(&mut rng, pmf, sample_space);
    let count = 10000;
    let mut reals = vec![0, 0, 0, 0];
    for _ in 0..count {
        reals[(random_variable_y.generate_realization() - 1.0) as usize] += 1;
    }
    for i in 0..reals.len() {
        println!("PMF of Y, {}: {}", i + 1, reals[i] as f32 / count as f32);
    }
}

pub fn test_continous_random_variable() {
    let cdf_of_exponential = |x: f64| -> f64 { 1.0 - f64::powf(std::f64::consts::E, -x / 12.0) };
    let mut inverse_cdf = |x: f64| -> f64 { -12.0 * (1.0 - x).ln() };
    let mut realizations: Vec<f64> = Vec::new();
    let mut rng = RandomNumberGenerator::new_default();
    let mut random_variable_z = ContinuousRandomVariableGenerator::new(&mut rng, &mut inverse_cdf);
    for i in 0..10000 {
        realizations.push(random_variable_z.generate_realization());
        println!("{}", realizations[i]);
    }

    let test_vals = vec![1f64, 2f64, 6f64, 12f64, 24f64];
    for test_val in test_vals {
        println!(
            "CDF of Z for {}, Expected: {}, Actual: {}",
            test_val,
            (cdf_of_exponential)(test_val),
            get_cdf(&realizations, test_val)
        );
    }
}
