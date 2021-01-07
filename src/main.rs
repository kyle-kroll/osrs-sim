use std::time::{Instant, Duration};
use structopt::StructOpt;
use rayon::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "OSRS-DS", about = "A drop simulator for Old School RuneScape!")]
struct Opt {
    #[structopt(short, long, default_value="10000")]
    iterations: usize,

    #[structopt(short, long, default_value="0.05")]
    droprate: f64
}

fn run_sim(rolls: &mut Vec<i32>, droprate: f64) {

    // Iterate through the vector and update each value based on the simulations
    rolls
        .par_iter_mut()
        .for_each(|i| {
            let mut num_rolls: i32 = 0;
            let mut running: bool = true;

            while running {
                num_rolls += 1;
                running = if fastrand::f64() <= droprate { false } else {true};
            }
            *i = num_rolls;
        });
}
fn main() {
    // Defining how many iterations I want to run. Ideally I will be able to run millions of iterations per minute
    let iterations: usize = Opt::from_args().iterations;
    
    // parse droprate from command line args
    let droprate: f64 = Opt::from_args().droprate;

    // Pre-allocate an array so it can be iterated through
    let mut rolls: Vec<i32> = vec![1; iterations];
    let start = Instant::now();
    run_sim(&mut rolls, droprate);
    let duration = start.elapsed();

    println!("Time to simulate {:?} iterations: {:?}", iterations, duration);
    println!("{:?} iterations/second", (iterations as f32 /Duration::as_secs_f32(&duration)) as f32);

    rolls.sort();
    let mid = rolls.len() / 2;
    println!("Median rolls needed to get the drop: {:?}", rolls[mid]);

    let nf: usize = (rolls.len() as f64 * 0.75) as usize;
    println!("75th percentile: {:?}", rolls[nf]);
}
