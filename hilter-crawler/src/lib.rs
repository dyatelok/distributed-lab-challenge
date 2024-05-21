#![feature(type_alias_impl_trait)]
use atomic_counter::*;
use futures::future;
use itertools::*;
use reqwest::{ClientBuilder, Method, Request, Response, Url};
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Write};
use std::iter;
use std::ops::Not;
use std::time::Duration;
use tower::{BoxError, Service, ServiceBuilder, ServiceExt};

pub struct Config {
    origin: String,
    pub destination: String,
    timeout: u64,
    pub max_depth: u32,
    limit: u64,
    fast: bool,
}

impl Config {
    pub fn new(
        origin: String,
        destination: String,
        timeout: u64,
        max_depth: u32,
        limit: u64,
        fast: bool,
    ) -> Self {
        Self {
            origin,
            destination,
            timeout,
            max_depth,
            limit,
            fast,
        }
    }
}

const DB_FILENAME: &str = "db.db";

struct DB {
    // references to which path in paths contains link. If link is contained in multiple paths - use the first one foung
    links_refs: HashMap<String, usize>,
    paths: Vec<Vec<String>>,
    file: File,
}

impl DB {
    fn new() -> Self {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(DB_FILENAME)
            .expect("Failed to open DB file");

        let mut buff = String::new();
        file.read_to_string(&mut buff)
            .expect("Failed to read from DB");

        let paths: Vec<Vec<String>> = buff
            .lines()
            .map(|line| line.split_whitespace().map(|s| s.to_owned()).collect())
            .collect();

        let links_refs = paths
            .iter()
            .enumerate()
            .flat_map(|(i, path)| path.iter().cloned().zip(iter::repeat(i)))
            .collect();

        Self {
            file,
            paths,
            links_refs,
        }
    }

    fn path_from(&self, link: &str) -> Option<Vec<&str>> {
        self.links_refs.get(link).map(|i| {
            self.paths[*i]
                .iter()
                .skip_while(|s| *s == link)
                .skip(1)
                .map(|s| s.as_str())
                .collect()
        })
    }

    fn add_path(&mut self, path: Vec<String>) {
        let path = path.join(" ") + "\n";
        self.file
            .write_all(path.as_bytes())
            .expect("Failed to write to DB");
    }
}

fn is_still_valid_path(path: &[&str]) -> bool {
    let not_a_path = path.iter().tuples().any(|(link, dest)| {
        let contains = reqwest::blocking::Client::new()
            .get(*link)
            .send()
            .and_then(|r| r.text())
            .map(|s| extract_links(&s).contains(*dest))
            .unwrap_or(false);

        contains.not()
    });

    not_a_path.not()
}

const WIKI_PREFIX: &str = "https://wikipedia.org";

fn normalize_url(url: &str) -> Option<String> {
    let mut url = match Url::parse(url) {
        Ok(parsed_url) => parsed_url
            .host_str()
            .filter(|host| host.contains("wikipedia"))
            .and_then(|_| {
                // Absolute url
                url.split("wiki")
                    .nth(2)
                    .map(|rest| [WIKI_PREFIX, "/wiki", rest].concat())
            }),
        Err(_) => {
            // Relative url
            if url.starts_with("/wiki/")
                && ![
                    "Wiki:",
                    "Wikipedia:",
                    "Special:",
                    "Help:",
                    "File:",
                    "Talk:",
                    "Category:",
                    "Portal:",
                    "Draft:",
                    "Module:",
                    "Template:",
                    "Template_talk:",
                ]
                .into_iter()
                .any(|substr| url.contains(substr))
            {
                Some([WIKI_PREFIX, url].concat())
            } else {
                None
            }
        }
    }?;

    if let Some(ind) = url.find('#') {
        url.truncate(ind);
    }

    Some(url)
}

fn extract_links(html: &str) -> HashSet<String> {
    Document::from(html)
        .find(Attr("id", "bodyContent").descendant(Name("a")))
        .filter_map(|n| n.attr("href"))
        .filter_map(normalize_url)
        .collect()
}

async fn fetch_url(mut client: ServiceClient, url: &str) -> Option<String> {
    let recuest = Request::new(Method::GET, url.parse().ok()?);
    let res = client.ready().await.ok()?.call(recuest).await.ok()?;
    // println!("Status for {}: {}", url, res.status());
    res.text().await.ok()
}

async fn find_urls<'a>(
    client: ServiceClient,
    (successful, failed): &(RelaxedCounter, RelaxedCounter),
    new_urls: impl IntoIterator<Item = String>,
) -> Vec<(String, HashSet<String>)> {
    let links = new_urls.into_iter().map(|url| {
        let cloned_cliend = client.clone();
        async {
            let html = fetch_url(cloned_cliend, &url).await;

            match html {
                Some(_) => successful.inc(),
                None => failed.inc(),
            };

            html.map(|html| extract_links(&html))
                .map(|outgoing| (url, outgoing))
        }
    });

    future::join_all(links)
        .await
        .into_iter()
        .flatten()
        .collect()
}

type ServiceClient = impl Service<Request, Response = Response, Error = BoxError> + Clone;

fn get_service(config: &Config) -> ServiceClient {
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(config.timeout))
        // .gzip(true)
        // .brotli(true)
        // .deflate(true)
        .build()
        .unwrap();

    ServiceBuilder::new()
        .buffer(1000)
        .rate_limit(config.limit, Duration::from_secs(1))
        .service(client)
}

pub async fn main_helper(
    config: Config,
    counters: &(RelaxedCounter, RelaxedCounter),
) -> Option<Vec<String>> {
    let mut db = if config.fast { Some(DB::new()) } else { None };

    let service = get_service(&config);

    let mut previous = HashMap::from([(config.origin.clone(), None::<String>)]);

    let mut visited = HashSet::new();
    let mut new_urls = HashSet::from([config.origin]);
    let mut depth = 1;

    while depth <= config.max_depth {
        visited.extend(new_urls.clone());

        let found_urls = find_urls(service.clone(), counters, new_urls).await;

        for (url, outgoing_links) in &found_urls {
            if outgoing_links.contains(&config.destination) {
                // build answer
                let mut url = Some(url);
                let mut path = vec![];

                while let Some(inner_url) = url {
                    path.push(inner_url.to_owned());

                    url = previous.get(inner_url).unwrap().as_ref();
                }
                path.reverse();

                if let Some(ref mut db) = db {
                    db.add_path(path.clone());
                }

                return Some(path);
            }

            for outgoing_link in outgoing_links {
                // println!("{outgoing_link}");

                if let Some(ref mut db) = db {
                    if let Some(path) = db.path_from(outgoing_link) {
                        if is_still_valid_path(&path) {
                            // build answer

                            let mut url = Some(url);
                            let mut vec = vec![];

                            while let Some(inner_url) = url {
                                vec.push(inner_url.to_owned());

                                url = previous.get(inner_url).unwrap().as_ref();
                            }
                            vec.reverse();
                            vec.extend(path.into_iter().map(|s| s.to_owned()));

                            db.add_path(vec.clone());

                            return Some(vec);
                        }
                    }
                }

                if outgoing_link != url {
                    let _ = previous
                        .entry(outgoing_link.to_owned())
                        .or_insert(Some(url.to_string()));
                }
            }
        }

        let found_urls = found_urls
            .into_iter()
            .flat_map(|(_, value)| value)
            .collect::<HashSet<_>>();

        new_urls = found_urls
            .difference(&visited)
            .map(|s| s.to_owned())
            .collect();

        println!("depth {depth}: found {} new urls", new_urls.len());
        depth += 1;
    }

    None
}
