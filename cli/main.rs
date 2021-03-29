use std::time::Instant;

use anyhow::Context;
// (Full example with detailed comments in examples/17_yaml.rs)
//
// This example demonstrates clap's building from YAML style of creating arguments which is far
// more clean, but takes a very small performance hit compared to the other two methods.
use clap::{load_yaml, App};
use reqwest::Client;
use sdmxblaze::{
    queries::metadata_query,
    reqwest_layer::Response,
    reqwest_warc::write_warc,
    structure::Structure,
    util::{filter_sources, read_sources},
};
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand() {
        Some(("crawl", sub_m)) => {
            println!("Crawling");
            let sources_file = sub_m.value_of("sources").unwrap();
            let mut sources = read_sources(sources_file)?;

            if sub_m.is_present("SOURCES") {
                let sourceIDs: Vec<_> = sub_m
                    .values_of("SOURCES")
                    .unwrap()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();
                sources = filter_sources(sources, sourceIDs)?
            }

            for source in sources {
                println!("{:#?}", source);

                let base_url =
                    Url::parse(format!("{}/", &source.url).as_str())?;

                let paths = ["dataflow", "all", "all"].to_vec();

                // Joining with a base url replaces the last item in path (public)
                let req_url =
                    base_url.join(metadata_query(paths).as_str())?.to_string();

                let start = Instant::now();
                let client = Client::new();
                let req = client
                    .get(&req_url)
                    .header("Accept", "application/json")
                    .build()?;
                let resp = client
                    .execute(
                        req.try_clone().context("Failed to clone request")?,
                    )
                    .await?;
                let duration = start.elapsed();

                let res = Response::parse(resp).await?;

                let wr_start = Instant::now();
                write_warc(req, res).await?;
                let wr_duration = wr_start.elapsed();

                println!(
                    "Req {:?} duration {:?}, write dur {:?}",
                    req_url, duration, wr_duration
                );
            }
            // sub_m.value_of("sources").ok_or("No sources file provided")
        } // clone was used
        Some(("push", sub_m)) => {}   // push was used
        Some(("commit", sub_m)) => {} // commit was used
        _ => {} // Either no subcommand or one not tested for...
    }

    Ok(())

    // Same as previous examples...
}
