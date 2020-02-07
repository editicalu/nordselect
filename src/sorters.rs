//! Sorters are ways to sort Servers, whereas the first one is the most likely to be selected for usage.

use super::servers::{Server, Servers};

use std;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;

use oping::Ping;

/// A Sorter is a way to order servers. It should be noted that these are sorted from best server
/// (according to the way of sorting) to worst one.
///
/// # Example
///
/// ```
/// use nordselect::{Servers, Server, sorters::Sorter};
/// use std::cmp::Ordering;
/// let mut data = Servers::dummy_data();
///
/// // Demo sorter: sorts on the alphabetical order of the countries of servers.
/// struct FlagSorter;
///
/// impl Sorter for FlagSorter {
///    fn sort(&self, a: &Server, b: &Server) -> Ordering {
///        a.flag.cmp(&b.flag)
///    }
/// }
///
/// data.sort(&FlagSorter);
/// // United Arab Emirates is the first country listed
/// assert_eq!(data.perfect_server().unwrap().flag, "AE");
///
/// ```
pub trait Sorter {
    /// Takes two servers, returns how they should be ordered according to this Sorter.
    fn sort(&self, _: &Server, _: &Server) -> Ordering;
}

/// Sorter that sorts servers based on their load, favouring the least loaded one.
pub struct LoadSorter;

impl Sorter for LoadSorter {
    fn sort(&self, a: &Server, b: &Server) -> Ordering {
        a.load.cmp(&b.load)
    }
}

/// Sorter that sorts based on a ping-test.
///
/// Please note that ping tests enhance the complexity of your program, whereas the `LoadSorter`
/// already provides very good results. I do not recommend you using this, as it requires special
/// privileges from the OS. These can be set using the following command on Linux.
///
/// ```bash
/// sudo setcap cap_net_raw+ep <your-compiled-binary>
/// ```
///
/// More details about this (and why you have to do it) can be found at the [oping crate](https://github.com/cfallin/rust-oping).
pub struct PingSorter {
    /// The results of the ping test.
    ping_results: HashMap<String, usize>,
}

/// Ways to set up a PingSorter.
impl PingSorter {
    /// Creates a new PingSorter using one ping instance, doing tests simultaneously. This is less precise, but is faster to run.
    ///
    /// This function takes an Iterator for Servers
    ///
    /// Returns an Error on failure.
    pub fn ping_single(
        servers: &Servers,
        tries: usize,
    ) -> Result<PingSorter, Box<dyn std::error::Error>> {
        let mut ping_results = HashMap::new();
        for _ in 0..tries {
            let mut pingr = Ping::new();
            for server in &servers.servers {
                pingr.add_host(server.domain.as_str())?;
            }

            let results = pingr.send()?;

            for result in results {
                let old_value: usize = *ping_results.get(&result.hostname).unwrap_or(&0usize);
                ping_results.insert(
                    result.hostname,
                    old_value + (result.latency_ms * 1000f64) as usize,
                );
            }
        }

        Ok(PingSorter {
            ping_results: HashMap::from_iter(
                ping_results
                    .into_iter()
                    .map(|(host, results)| (host, results / tries)),
            ),
        })
    }

    /// Creates a new PingSorter using a ping instance for every server, doing tests after one another. This is more precise, but takes significantly longer.
    ///
    /// This function takes an Iterator for Servers
    ///
    /// Returns an Error on failure.
    pub fn ping_multi(
        servers: &Servers,
        tries: usize,
    ) -> Result<PingSorter, Box<dyn std::error::Error>> {
        let mut ping_results = HashMap::new();
        for server in &servers.servers {
            let mut sum = 0;
            for _ in 0..tries {
                let mut pingr = Ping::new();
                pingr.add_host(server.domain.as_str())?;
                sum += (pingr.send()?.next().unwrap().latency_ms * 1000f64) as usize;
            }
            ping_results.insert(server.domain.clone(), sum / tries);
        }

        Ok(PingSorter { ping_results })
    }
}

impl Sorter for PingSorter {
    fn sort(&self, a: &Server, b: &Server) -> Ordering {
        self.ping_results
            .get(a.domain.as_str())
            .expect("Server not found in ping result")
            .cmp(
                self.ping_results
                    .get(b.domain.as_str())
                    .expect("Other server not found in ping result"),
            )
    }
}
