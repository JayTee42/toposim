use std::iter::{once, repeat};
use std::ops::Div;
use std::str::FromStr;

use rayon::prelude::*;
use structopt::StructOpt;

/// The different network topologies we support
enum Topology {
    Ring,
    OnewayRing,
    Star,
    Line,
}

impl FromStr for Topology {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Topology::*;

        Ok(match s {
            "ring" => Ring,
            "oneway_ring" => OnewayRing,
            "star" => Star,
            "line" => Line,
            _ => return Err("Unknown topology type (valid ones are \"ring\", \"oneway_ring\", \"star\", \"line\")"),
        })
    }
}

/// CLI arguments
#[derive(StructOpt)]
#[structopt()]
struct Opt {
    #[structopt(name = "TOPOLOGY")]
    topology: Topology,

    #[structopt(name = "COUNT")]
    n: usize,
}

// How many simulations to run?
const SIM_COUNT: usize = 1000000;

/// This is the cost matrix of a network topology graph.
struct CostMatrix {
    /// The number of nodes in the graph
    n: usize,

    /// The cost entries (row-major)
    cost: Vec<usize>,
}

impl CostMatrix {
    /// Create a ring topology.
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

    /// Create a directed ring topology.
    pub fn oneway_ring(n: usize) -> Self {
        assert!(n > 1, "n must be > 1.");

        Self {
            n,
            cost: (0..n)
                .flat_map(|i| (1..n).cycle().skip(n - i - 1).take(n - 1))
                .collect(),
        }
    }

    /// Create a star topology.
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

    /// Create a line topology.
    pub fn line(n: usize) -> Self {
        assert!(n > 1, "n must be > 1.");

        Self {
            n,
            cost: (0..n)
                .flat_map(|i| (1..(i + 1)).rev().chain(1..(n - i)))
                .collect(),
        }
    }

    /// Let each node randomly select a target node to send a message to.
    /// Return the average message hop count over all nodes.
    pub fn simulate_step(&self) -> f64 {
        let hop_sum: f64 = (0..self.n)
            .map(|row| self.cost[(row * (self.n - 1)) + fastrand::usize(..(self.n - 1))] as f64)
            .sum();

        (hop_sum as f64) / (self.n as f64)
    }
}

fn main() {
    use Topology::*;

    // Build the cost matrix from the args.
    let opt = Opt::from_args();

    if opt.n <= 1 {
        println!("Number of nodes (COUNT) must be > 1.");
        return;
    }

    let cost = match opt.topology {
        Ring => CostMatrix::ring(opt.n),
        OnewayRing => CostMatrix::oneway_ring(opt.n),
        Star => CostMatrix::star(opt.n),
        Line => CostMatrix::line(opt.n),
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
