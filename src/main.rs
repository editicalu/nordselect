extern crate clap;
extern crate nordselect;

use nordselect::bench::Benchmarker;
use nordselect::bench::LoadBenchmarker;
use nordselect::Servers;

mod cli_help;
use cli_help::*;

// TODO: sort
fn sort(data: &mut Servers, matches: &clap::ArgMatches) {
    use std::collections::HashMap;

    let bencher = LoadBenchmarker {};

    // TODO: use matches to find out which benchmarker to use
    let mut bench_scores = HashMap::new();
    {
        data.servers
            .iter()
            .map(|server| (server, bencher.bench(server)))
            .filter(|(_, bench_result)| bench_result.is_ok())
            .map(|(server, bench_result)| (server, bench_result.unwrap()))
            .for_each(|tuple| {
                // TODO: fix
                bench_scores.insert(tuple.0.domain.clone(), tuple.1);
            });
    }

    let bench_scores = bench_scores;
    data.servers.sort_by(|server_a, server_b| {
        bench_scores[&server_a.domain].cmp(&bench_scores[&server_b.domain])
    });
}

#[tokio::main]
async fn main() {
    let data_future = Servers::from_api();
    // Parse CLI args
    let matches = parse_cli_args();

    let show_filters = matches.is_present("list_filters");
    // Detect filters
    let filters_to_apply = parse_filters(&matches);

    // Get API data
    let mut data = match data_future.await {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Could not download data: {}", x);
            std::process::exit(1);
        }
    };

    // Should we only show the available filters?
    if show_filters {
        show_available_filters(&data);
        std::process::exit(0);
    }

    // Filter servers that are not required.
    let filters = filters_to_apply.await;
    if let Err(error) = filters {
        eprintln!("An error occurred when parsing filters: {}", error);
        std::process::exit(1);
    }

    let filters = filters.unwrap();

    apply_filters(filters, &mut data);

    // Sort the servers
    sort(&mut data, &matches);

    // Print the ideal server, if found.
    if let Some(server) = data.perfect_server() {
        println!(
            "{}",
            if matches.is_present("domain") {
                &server.domain
            } else {
                server.name().unwrap_or(&server.domain)
            }
        );
    } else {
        eprintln!("No server found");
        std::process::exit(1);
    }
}
