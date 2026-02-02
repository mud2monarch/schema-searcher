use log::info;
use reqwest::Client;
use schema_searcher::bigquery::types::Dataset;
use yup_oauth2::read_service_account_key;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GBQ authentication and Reqwest client boilerplate
    let service_account_key: yup_oauth2::ServiceAccountKey =
        read_service_account_key("zw_personal_service_acc.json").await?;
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

    // finish bigquery and Reqwest boilerplate
    let res = client
        .get("https://bigquery.googleapis.com/bigquery/v2/projects/bigquery-public-data/datasets/")
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", res);

    Ok(())
}
