use reqwest::Client;

use crate::{queries::metadata_query, reqwest_warc::write_response};
use anyhow::Result;
use std::time::Instant;
use std::{convert::TryFrom, string::ToString};
use url::Url;

pub async fn get_dataflows<T: ToString>(endpoint: T) -> Result<()> {
    let paths = ["dataflow", "all", "all", "latest"].to_vec();

    let base_url = endpoint.to_string();

    // Joining with a base url replaces the last item in path (public)
    let path_url = Url::try_from(base_url.as_str())?
        .join(metadata_query(paths).as_str())?;

    let url = path_url.to_string();

    let start = Instant::now();
    let resp = Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?;

    let duration = start.elapsed();

    let wr_start = Instant::now();
    write_response(resp).await?;
    let wr_duration = wr_start.elapsed();

    println!(
        "Req {}, {:?}, write dur {:?}",
        base_url, duration, wr_duration
    );
    Ok(())
}

pub async fn get_dataflow<T: ToString>(endpoint: T, dataflow: T) -> Result<()> {
    let ds = dataflow.to_string();
    let paths = ["dataflow", "all", ds.as_str(), "latest"].to_vec();

    let base_url = endpoint.to_string();

    // Joining with a base url replaces the last item in path (public)
    let path_url = Url::try_from(base_url.as_str())?
        .join(metadata_query(paths).as_str())?;

    let url = path_url.to_string();

    let start = Instant::now();
    let resp = Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?;

    write_response(resp).await?;

    let duration = start.elapsed();

    println!("Req {}, {:?}", base_url, duration);
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::util::get_source;

    use super::*;

    // #[tokio::test]
    // async fn get_random_dataflow() -> Result<()> {
    //     get_dataflows("https://google.com/").await?;
    //     Ok(())
    // }

    #[tokio::test]
    async fn get_dataflow_details() -> Result<()> {
        let source = get_source("IMF")?;
        get_dataflows(source.url + "/").await?;
        Ok(())
    }
    //
    // let TESTING_APIS: Vec<&str> = vec!["IMF", "WB", "WB_WDI"];
}
