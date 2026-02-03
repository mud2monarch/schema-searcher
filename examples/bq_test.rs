use log::info;
use schema_searcher::bigquery::{
    client::{authenticate, get_datasets, get_tables},
    types::DatasetList,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = authenticate("zw_personal_service_acc.json").await?;

    let project = "bigquery-public-data";

    let tables = get_datasets(&client, project).await.unwrap();

    println!("{:?}", tables);

    Ok(())
}
