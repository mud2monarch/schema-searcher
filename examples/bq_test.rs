use log::info;
use schema_searcher::bigquery::{client::authenticate, types::DatasetList};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = authenticate("zw_personal_service_acc.json").await?;

    let datasets = client
        .get("https://bigquery.googleapis.com/bigquery/v2/projects/bigquery-public-data/datasets/")
        .send()
        .await?
        .json::<DatasetList>()
        .await?;

    println!("{:?}", datasets);

    Ok(())
}
