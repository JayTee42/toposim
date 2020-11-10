use rayon::prelude::*;
use std::iter::{once, repeat};
use std::ops::Div;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "moepp")]
struct Opt {
    #[structopt(name = "TOPOLOGY")]
    topo: String,

    #[structopt(name = "COUNT")]
    n: usize,
}

// How many simulations to run?
const SIM_COUNT: usize = 1000000;

struct CostMatrix {
    n: usize,
    cost: Vec<usize>,
}

impl CostMatrix {
    pub fn ring(n: usize) -> Self {
        assert!(n > 1, "n must be > 1.");

        Self {
            n,
            cost: (0..n)
                .flat_map(|i| {
                    (1..((n + 1) / 2))
                        .chain(if (n % 2) == 0 { Some(n / 2) } else { None })
                        .chain((1..((n + 1) / 2)).rev())
                        .cycle()
                        .skip(n - i - 1)
                        .take(n - 1)
                })
                .collect(),
        }
    }

    pub fn oneway_ring(n: usize) -> Self {
        assert!(n > 1, "n must be > 1.");

        Self {
            n,
            cost: (0..n)
                .flat_map(|i| (1..n).cycle().skip(n - i - 1).take(n - 1))
                .collect(),
        }
    }

    pub fn star(n: usize) -> Self {
        assert!(n > 1, "n must be > 1.");

        Self {
            n,
            cost: repeat(1)
                .take(n - 1)
                .chain(
                    once(1)
                        .chain(repeat(2).take(n - 2))
                        .cycle()
                        .take((n - 1) * (n - 1)),
                )
                .collect(),
        }
    }

    pub fn line(n: usize) -> Self {
        assert!(n > 1, "n must be > 1.");

        Self {
            n,
            cost: (0..n)
                .flat_map(|i| (1..(i + 1)).rev().chain(1..(n - i)))
                .collect(),
        }
    }

    pub fn simulate_step(&self) -> f64 {
        let hop_sum: f64 = (0..self.n)
            .map(|row| self.cost[(row * (self.n - 1)) + fastrand::usize(..(self.n - 1))] as f64)
            .sum();

        (hop_sum as f64) / (self.n as f64)
    }
}

fn main() {
    // Build the cost matrix from the args.
    let opt = Opt::from_args();

    if opt.n <= 1 {
        println!("Number of nodes (COUNT) must be > 1.");
        return;
    }

    let cost = match opt.topo.to_lowercase().as_str() {
        "ring" => CostMatrix::ring(opt.n),
        "oneway_ring" => CostMatrix::oneway_ring(opt.n),
        "star" => CostMatrix::star(opt.n),
        "line" => CostMatrix::line(opt.n),
        _ => {
            println!("Unknown topology type (valid ones are \"ring\", \"oneway_ring\", \"star\", \"line\")");
            return;
        }
    };

    // Perform the simulation (in parallel because we can).
    let result = (0..SIM_COUNT)
        .into_par_iter()
        .map(|_| cost.simulate_step())
        .sum::<f64>()
        .div(SIM_COUNT as f64);

    // Print the result.
    println!(
        "Average hop count after {} simulation steps: {:.3} hops",
        SIM_COUNT, result
    );
}
