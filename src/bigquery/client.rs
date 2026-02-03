use log::info;
use reqwest::Client;
use yup_oauth2::read_service_account_key;

use crate::bigquery::types::{DatasetList, DatasetReference, TableList, TableReference};

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

pub async fn get_datasets(
    client: &Client,
    project_id: &str,
) -> Result<Vec<DatasetReference>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://bigquery.googleapis.com/bigquery/v2/projects/{}/datasets",
        project_id
    );
    let response = client.get(&url).send().await?.json::<DatasetList>().await?;

    let datasets = response
        .datasets
        .into_iter()
        .map(|dataset| dataset.dataset_reference)
        .collect();

    Ok(datasets)
}

pub async fn get_tables(
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
