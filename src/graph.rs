use std::convert::AsRef;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct Graph {
    nodes_count: usize,
    adjacency: Vec<bool>,
}

impl Graph {
    /// Query the number of nodes in the graph.
    #[inline(always)]
    pub fn nodes_count(&self) -> usize {
        self.nodes_count
    }

    /// Test if the nodes with the given indices are adjacent.
    #[inline(always)]
    pub fn adjacent(&self, first: usize, second: usize) -> bool {
        self.adjacency[(first * self.nodes_count) + second]
    }

    // Load a graph from a 0-1 adjacency matrix file.
    pub fn load_from<P: AsRef<Path>>(path: P) -> io::Result<Graph> {
        // Open the file.
        let mut lines = BufReader::new(File::open(path)?).lines();

        // Parse the first line. It dictates the number of nodes.
        let first_line = lines.next().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Graph file must not be empty.",
        ))??;

        let mut adjacency = Vec::new();
        let nodes_count = Self::parse_line(&first_line, &mut adjacency)?;

        // Read the rest of the lines.
        for line_res in lines {
            // Check and trim the current line.
            let untrimmed_line = line_res?;
            let line = untrimmed_line.trim();

            // Skip empty lines.
            if line.is_empty() {
                continue;
            }

            // The number of entries per row must be equal.
            let curr_count = Self::parse_line(&line, &mut adjacency)?;

            if curr_count != nodes_count {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Adjacency matrix row lengths must be equal.",
                ));
            }
        }

        Ok(Self {
            nodes_count,
            adjacency,
        })
    }

    #[inline(always)]
    fn parse_line(line: &str, adjacency: &mut Vec<bool>) -> io::Result<usize> {
        let mut count = 0;

        for c in line.chars().filter(|c| !c.is_whitespace()) {
            match c {
                '0' => adjacency.push(false),
                '1' => adjacency.push(true),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Graph file may only contain 0 and 1.",
                    ))
                }
            }

            count += 1;
        }

        if count < 1 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Graph must not be empty.",
            ))
        } else {
            Ok(count)
        }
    }

    pub fn calculate_shortest_paths(&self) -> Vec<Option<usize>> {}
}
