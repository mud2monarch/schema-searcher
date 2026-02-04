#![allow(unused_imports)]

use log::info;
use schema_searcher::bigquery::{
    client::{authenticate, get_datasets, list_project_tables},
    types::DatasetList,
};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = authenticate("zw_personal_service_acc.json").await?;

    let project = "bigquery-public-data";

    let tables = list_project_tables(&client, project).await.unwrap();

    println!("{:?}", tables);

    Ok(())
}
