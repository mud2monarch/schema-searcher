#![allow(unused_imports)]

use futures::future::join_all;
use log::{info, warn};
use schema_searcher::bigquery::{
    client::{authenticate, get_table_columns, list_project_datasets, list_project_tables},
    types::DatasetList,
};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let client = authenticate("zw_personal_service_acc.json").await?;

    let project = "bigquery-public-data";

    let tables = list_project_tables(&client, project).await.unwrap();

    let futures = tables.iter().map(|table| get_table_columns(&client, table));
    let results = join_all(futures).await;

    for result in results.into_iter().take(5) {
        match result {
            Ok(columns) => info!("Columns: {:?}", columns),
            Err(err) => warn!("Error: {}", err),
        }
    }
    std::process::exit(0);
    Ok(())
}
