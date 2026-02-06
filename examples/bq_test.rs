#![allow(unused_imports)]

use clap::Parser;
use futures::future::join_all;
use log::{info, warn};
use schema_searcher::{
    bigquery::{
        client::{authenticate, get_table_columns, list_project_datasets, list_project_tables},
        types::DatasetList,
    },
    io::cli,
};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = cli::Args::parse();
    let creds_path: &str = args.creds_path.as_str();
    // let output_file_path: &str = args.output_file.as_str();
    let project = args.project.as_str();

    let client = authenticate(creds_path).await?;

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
