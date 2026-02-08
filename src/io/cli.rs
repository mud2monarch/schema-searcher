use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// The filepath to the service account credentials file.
    /// Defaults to "service_account_creds.json" in the current working directory.
    #[arg(short, long, default_value = "service_account_creds.json")]
    pub creds_path: String,

    /// The project from which you want to search for tables.
    #[arg(short, long, default_value = "bigquery-public-data")]
    pub project: String,

    /// The filepath to which to write the final output file.
    /// Defaults to "tables.txt" in the current working directory.
    #[arg(short, long, default_value = "tables.txt")]
    pub output_path: String,
}
