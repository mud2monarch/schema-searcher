use clap::Parser;
use schema_searcher::{
    bigquery::client::{authenticate, get_tables, list_project_tables},
    io::writer::write_table,
    io::{cli, fuzzy},
};
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = cli::Args::parse();
    let creds_path: &str = args.creds_path.as_str();
    let output_path: &str = args.output_path.as_str();
    let project = args.project.as_str();

    let client = authenticate(creds_path).await?;

    let table_names = list_project_tables(&client, project).await.unwrap();

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
