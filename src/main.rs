extern crate clap;
extern crate nordselect;

use nordselect::bench::Benchmarker;
use nordselect::bench::LoadBenchmarker;
use nordselect::filters::{self, Filter};
use nordselect::{ServerCategory, Servers};
use std::collections::HashSet;

mod cli_help;
use cli_help::*;

fn consider_negating_filter<'a>(filter: &'a str) -> (&'a str, bool) {
    if filter.len() > 0 && &filter[..1] == "!" {
        return (&filter[1..], true);
    }
    (filter.into(), false)
}

#[test]
fn consider_negating_filter_test() {
    assert_eq!(consider_negating_filter("qwe"), ("qwe", false));
    assert_eq!(consider_negating_filter("!qwe"), ("qwe", true));
    assert_eq!(consider_negating_filter(""), ("", false));
}

fn parse_filters(cli_filters: clap::Values, data: &Servers) -> Vec<Box<dyn Filter>> {
    // Parse which countries are in the data
    let flags = data.flags();

    let mut lib_filters: Vec<Box<dyn Filter>> = Vec::new();
    let mut category_filter_added = false;
    let mut included_countries = HashSet::new();
    let mut excluded_countries = HashSet::new();

    for original_filter in cli_filters.into_iter() {
        let (filter, is_negating) = consider_negating_filter(original_filter);

        if let Some((lib_filter, is_category_filter)) = parse_static_filter(filter) {
            lib_filters.push(if is_negating {
                Box::new(filters::NegatingFilter::from(lib_filter))
            } else {
                lib_filter
            });
            if is_category_filter {
                category_filter_added = true;
            }
            continue;
        }

        let filter_upper = filter.to_uppercase();
        let contries_to_modify = if is_negating {
            &mut excluded_countries
        } else {
            &mut included_countries
        };

        if flags.contains(filter_upper.as_str()) {
            contries_to_modify.insert(filter_upper);
            continue;
        }

        if let Some(region_countries) = filters::Region::from_str(&filter_upper) {
            region_countries.countries().into_iter().for_each(|flag| {
                contries_to_modify.insert(flag.into());
                ()
            });
            continue;
        }

        if let Ok(binary) = std::env::current_exe()
            .unwrap()
            .into_os_string()
            .into_string()
        {
            eprintln!(
                "Error: unknown filter: \"{}\". Run `{} --filters` to list all available filters.",
                original_filter, binary
            );
        } else {
            eprintln!(
                "Error: unknown filter: \"{}\". Use `--filters` to list all available filters.",
                original_filter
            );
        }
        std::process::exit(1);
    }

    // Use a Standard server if no special server is requested.
    if !category_filter_added {
        lib_filters.push(Box::new(filters::CategoryFilter::from(
            ServerCategory::Standard,
        )));
    }

    // Add countries filters.
    if !included_countries.is_empty() {
        lib_filters.push(Box::new(filters::CountriesFilter::from(included_countries)));
    }
    if !excluded_countries.is_empty() {
        lib_filters.push(Box::new(filters::NegatingFilter::new(
            filters::CountriesFilter::from(excluded_countries),
        )));
    }

    lib_filters
}

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

fn main() {
    // Parse CLI args
    let matches = parse_cli_args();

    // Get API data
    let mut data = match Servers::from_blocking_api() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Could not download data: {}", x);
            std::process::exit(1);
        }
    };

    // Should we only show the available filters?
    if matches.is_present("list_filters") {
        show_available_filters(&data);
        std::process::exit(0);
    }

    // Detect filters
    let filters_to_apply = parse_filters(
        matches
            .values_of("filter")
            .unwrap_or(clap::Values::default()),
        &data,
    );

    // Filter servers that are not required.
    apply_filters(filters_to_apply, &mut data);

    // Sort the servers
    sort(&mut data, &matches);

    // Print the ideal server, if found.
    if let Some(server) = data.perfect_server() {
        println!(
            "{}",
            match matches.is_present("domain") {
                true => &server.domain,
                false => server.name().unwrap_or(&server.domain),
            }
        );
    } else {
        eprintln!("No server found");
        std::process::exit(1);
    }
}
