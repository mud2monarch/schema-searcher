use futures::future::{join_all, try_join_all};
#[allow(unused_imports)]
use log::{info, warn};
use reqwest::Client;
use yup_oauth2::read_service_account_key;

use crate::bigquery::types::{
    Column, DatasetList, DatasetReference, Table, TableList, TableReference,
};

pub async fn authenticate(creds_path: &str) -> Result<Client, Box<dyn std::error::Error>> {
    // GBQ authentication and Reqwest client boilerplate
    let service_account_key: yup_oauth2::ServiceAccountKey =
        read_service_account_key(creds_path).await?;
    let authenticator = yup_oauth2::ServiceAccountAuthenticator::builder(service_account_key)
        .build()
        .await?;

    let token = authenticator
        .token(&["https://www.googleapis.com/auth/bigquery.readonly"])
        .await?;

    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", token.token().unwrap())
            .parse()
            .unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}

// FYI here is the flow of the BigQuery API:
// method: dataset.list (projectID) -> Vec<Dataset>; Dataset contains a DatasetReference.
// method: table.list (projectID and a datasetID) -> Vec<Table>; Table contains a TableReference.
// method: table.get (projectID, datasetID, tableID) -> Schema

async fn list_project_datasets(
    client: &Client,
    project_id: &str,
) -> Result<Vec<DatasetReference>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://bigquery.googleapis.com/bigquery/v2/projects/{}/datasets",
        project_id
    );
    let response = client.get(&url).send().await?.json::<DatasetList>().await?;

    let datasets: Vec<DatasetReference> = response
        .datasets
        .into_iter()
        .map(|dataset| dataset.dataset_reference)
        .collect();

    Ok(datasets)
}

async fn list_dataset_tables(
    client: &Client,
    dataset: &DatasetReference,
) -> Result<Vec<TableReference>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://bigquery.googleapis.com/bigquery/v2/projects/{}/datasets/{}/tables",
        dataset.project_id, dataset.dataset_id
    );
    let response = client.get(&url).send().await?.json::<TableList>().await?;

    let tables = response
        .tables
        .into_iter()
        .map(|table| table.table_reference)
        .collect();

    Ok(tables)
}

pub async fn list_project_tables(
    client: &Client,
    project_id: &str,
) -> Result<Vec<TableReference>, Box<dyn std::error::Error>> {
    let datasets = list_project_datasets(client, project_id).await?;

    let futures = datasets
        .iter()
        .map(|dataset| list_dataset_tables(client, dataset));

    // TODO: Use join_all and flatten. don't need to mass fail on one api call here.
    let tables = try_join_all(futures).await?.into_iter().flatten().collect();

    Ok(tables)
}

async fn get_table(
    client: &Client,
    table_id: &TableReference,
) -> Result<Table, Box<dyn std::error::Error>> {
    let url = format!(
        "https://bigquery.googleapis.com/bigquery/v2/projects/{}/datasets/{}/tables/{}",
        table_id.project_id, table_id.dataset_id, table_id.table_id
    );

    let table: Table = client.get(&url).send().await?.json::<Table>().await?;

    Ok(table)
}

pub async fn get_tables(
    client: &Client,
    table_ids: &Vec<TableReference>,
) -> Result<Vec<Table>, Box<dyn std::error::Error>> {
    let futures = table_ids.into_iter().map(|table| get_table(client, table));

    let tables = join_all(futures)
        .await
        .into_iter()
        .filter_map(|table| match table {
            Ok(table) => Some(table),
            Err(err) => {
                warn!("Failed to get table: {}", err);
                None
            }
        })
        .collect::<Vec<Table>>();

    Ok(tables)
}
