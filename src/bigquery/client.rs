use log::info;
use reqwest::Client;
use yup_oauth2::read_service_account_key;

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
