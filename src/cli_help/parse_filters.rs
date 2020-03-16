use nordselect::filters::{BlackListFilter, Filter, WhiteListFilter};
use std::error::Error;

async fn blacklist(args: &clap::ArgMatches<'_>) -> Result<Option<BlackListFilter>, Box<dyn Error>> {
    let blacklist_sources = args.values_of("blacklist");
    if let Some(mut sources) = blacklist_sources {
        let source = sources.next().unwrap();
        let first_filter = if urlparse::urlparse(source).scheme.is_empty() {
            BlackListFilter::from_file(source).await?
        } else {
            BlackListFilter::from_url(source).await?
        };

        for _ in sources {
            // TODO: add support for multiple blacklists
        }

        let filter = first_filter;
        Ok(Some(filter))
    } else {
        Ok(None)
    }
}

async fn whitelist(args: &clap::ArgMatches<'_>) -> Result<Option<WhiteListFilter>, Box<dyn Error>> {
    let whitelist_sources = args.values_of("whitelist");
    if let Some(mut sources) = whitelist_sources {
        let source = sources.next().unwrap();
        let first_filter = if urlparse::urlparse(source).scheme.is_empty() {
            WhiteListFilter::from_file(source).await?
        } else {
            WhiteListFilter::from_url(source).await?
        };

        for _ in sources {
            // TODO: add support for multiple whitelists
        }

        let filter = first_filter;
        Ok(Some(filter))
    } else {
        Ok(None)
    }
}

// TODO: whitelists
async fn parse_lists(
    args: &clap::ArgMatches<'_>,
) -> Result<(Option<WhiteListFilter>, Option<BlackListFilter>), Box<dyn Error>> {
    let blacklist = blacklist(args);
    let whitelist = whitelist(args);
    Ok((whitelist.await?, blacklist.await?))
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
