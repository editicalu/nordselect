use nordselect::filters::*;
use std::error::Error;

// TODO: blacklists
async fn parse_lists(
    args: &clap::ArgMatches<'_>,
) -> Result<(Option<WhiteListFilter>, Option<BlackListFilter>), Box<dyn Error>> {
    let blacklists = args.values_of("blacklist").and_then(|paths| {});

    let whitelists = args
        .values_of("whitelist")
        .and_then(|paths| paths.flat_map(|path| ));

    Ok((None, None))
}

pub async fn parse_filters(
    args: &clap::ArgMatches<'_>,
) -> Result<Vec<Box<dyn Filter>>, Box<dyn Error>> {
    // TODO: this
    // We assume that every filter that we do not recognize is a flag filter. We will warn the user when a new flag was ound.
    let filters_args = args.values_of("filter").unwrap_or_default();
    let lists_future = parse_lists(args);

    let mut filters: Vec<Box<dyn Filter>> = Vec::new();
    let (whitelist_opt, blacklist_opt) = lists_future.await?;
    if let Some(whitelist) = whitelist_opt {
        filters.insert(0, Box::new(whitelist));
    }
    if let Some(blacklist) = blacklist_opt {
        filters.insert(0, Box::new(blacklist));
    }

    Ok(filters)
}
