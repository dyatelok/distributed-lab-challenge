use atomic_counter::*;
use clap::Parser;
use const_format::concatcp;
use hilter_crawler::{main_helper, Config};
use std::time::Instant;

const DEFAULT_DESTINATION: &str = "https://wikipedia.org/wiki/Adolf_Hitler";
const DEFAULT_TIMEOUT: u64 = 20;
const DEFAULT_MAX_DEPTH: u32 = 6;
const DEFAULT_LIMIT: u64 = 20;
const DEFAULT_FAST: bool = true;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Origin to start crawling
    #[arg(short, long)]
    origin: String,

    #[arg(short, long, help = concatcp!("Destination to crawl to. Defaults to \"", DEFAULT_DESTINATION, '"'))]
    destination: Option<String>,

    #[arg(short, long, help = concatcp!("Reqwest timeout in seconds. Defaults to ", DEFAULT_TIMEOUT))]
    timeout: Option<u64>,

    #[arg(short, long, help = concatcp!("Maximal depth to crawl. Defaults to ", DEFAULT_MAX_DEPTH))]
    #[arg(short, long)]
    max_depth: Option<u32>,

    #[arg(short, long, help = concatcp!("Limit of recuests per second. Defaults to ", DEFAULT_LIMIT),)]
    #[arg(short, long)]
    limit: Option<u64>,

    #[arg(short, long, help = concatcp!("If true - program will create a database and check if any reached link is contained in database. If database contains link, it will check if path is still reachable and return it. This may lead to non-optimal (not shortest length) paths. Defaults to ", DEFAULT_FAST))]
    fast: Option<bool>,
}

fn get_config() -> Config {
    let cli = Cli::parse();

    let origin = cli.origin;
    let timeout = cli.timeout.unwrap_or(DEFAULT_TIMEOUT);
    let max_depth = cli.max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
    let limit = cli.limit.unwrap_or(DEFAULT_LIMIT);
    let fast = cli.fast.unwrap_or(DEFAULT_FAST);

    if fast && cli.destination.is_some() {
        panic!("DB now only works with Default destination");
    }

    let destination = cli.destination.unwrap_or(DEFAULT_DESTINATION.to_owned());

    Config::new(origin, destination, timeout, max_depth, limit, fast)
}

#[tokio::main]
async fn main() {
    let config = get_config();
    let max_depth = config.max_depth;
    let destination = config.destination.clone();

    let counters = (RelaxedCounter::new(0), RelaxedCounter::new(0));

    let now = Instant::now();
    let res = main_helper(config, &counters).await;

    match res {
        Some(mut path) => {
            let depth = path.len();
            path.push(destination);

            println!("\nDestination found in {depth} steps\n");

            let path = path.join(" ->\n");
            println!("{path}");
        }
        None => println!("Destination is not reached in {max_depth} steps"),
    }

    let successful = counters.0.get();
    let failed = counters.1.get();

    let elapsed = now.elapsed().as_millis();

    println!(
        "\nFinished in {elapsed:6} ms
Successfully fetched {successful:6} links
Failed to fetch     {failed:6} links"
    );
}
