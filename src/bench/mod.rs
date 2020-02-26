use crate::Server;

/// A Benchmarker is a way to say how good a given server is and whether it should be considered for a connection.
pub trait Benchmarker {
    /// Calculates a score for the given server. A lower score is better.
    fn bench(server: &Server) -> u32;
}

pub trait ParallelBenchmarker: Benchmarker {}
