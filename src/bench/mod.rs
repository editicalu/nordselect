use crate::Server;

mod load;
mod ping;
pub use self::load::LoadBenchmarker;

/// A Benchmarker is a way to say how good a given server is and whether it should be considered for a connection.
pub trait Benchmarker {
    /// Calculates a score for the given server. A lower score is better.
    fn bench(&self, server: &Server) -> u32;
}

/// A trait that indicates that this Benchmarker can be run in parallel, whichout having an effect on the result.LoadBenchmarker
///
/// This should be implemented when building a
pub trait ParallelBenchmarker: Benchmarker {}

/// LoggableBenchmarker means that this Benchmarker can be run with a verbose flag to indicate that we would like to see the raw measurement values.
pub trait LoggableBenchmarker<T> {
    fn bench_log(&self, server: &Server) -> (u32, T);
}

impl<T> Benchmarker for dyn LoggableBenchmarker<T> {
    fn bench(&self, server: &Server) -> u32 {
        self.bench_log(server).0
    }
}
