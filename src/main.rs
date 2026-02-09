use colorize::AnsiColor;
use inquire::Text;
use schema_searcher::{
    bigquery::client::{authenticate, get_tables, list_project_tables},
    io::fuzzy,
    io::writer::write_table,
};
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let creds_path = Text::new("Enter your credentials path".yellow().as_str())
        .with_default("service_account_creds.json")
        .prompt()?;
    let project_id = Text::new("Enter your project ID".yellow().as_str())
        .with_default("bigquery-public-data")
        .prompt()?;
    let output_path = Text::new(
        "What would you like the output file to be called?"
            .yellow()
            .as_str(),
    )
    .with_default("tables.txt")
    .prompt()?;

    let client = authenticate(creds_path.as_str()).await?;

    let table_names = list_project_tables(&client, project_id.as_str())
        .await
        .unwrap();

    let desired_tables =
        tokio::task::spawn_blocking(move || fuzzy::collect_tables(table_names)).await?;

    let tables_w_schemas = get_tables(&client, &desired_tables).await?;

    let mut output_file = File::create(output_path).expect("Failed to create output file");

    for table in tables_w_schemas {
        if let Err(err) = write_table(&mut output_file, &table) {
            eprintln!(
                "Failed to write table {}: {}",
                table.table_reference.to_str(),
                err
            );
        }
    }

    Ok(())
}
