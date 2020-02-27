use crate::Server;

mod load;
mod ping;
mod prelude;
pub use self::load::LoadBenchmarker;

pub type ScoreLogResult<T> = Result<(u32, T), Box<dyn std::error::Error>>;

/// A Benchmarker is a way to say how good a given server is and whether it should be considered for a connection.
pub trait Benchmarker<T> {
    /// Calculates a score for the given server. A lower score is better.
    fn bench(&self, server: &Server) -> ScoreLogResult<T>;
}

/// A trait that indicates that this Benchmarker can be run in parallel, whichout having an effect on the result.LoadBenchmarker
///
/// This should be implemented when building a
pub trait ParallelBenchmarker<T>: Benchmarker<T> {}
