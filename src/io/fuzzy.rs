use crate::bigquery::types::TableReference;
use skim::prelude::*;
use std::io::Cursor;

fn tables_to_string(tables: Vec<TableReference>) -> String {
    tables
        .iter()
        .map(|table: &TableReference| table.to_str())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn collect_tables(tables: Vec<TableReference>) -> Vec<TableReference> {
    let options = SkimOptionsBuilder::default()
        .height(String::from("80%"))
        .multi(true)
        .header(Some(String::from(
            "Select tables. Use tab/shift-tab to add/remove items. Press Enter to confirm.",
        )))
        .build()
        .unwrap();

    let input = tables_to_string(tables);

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(options, Some(items)).map(|out| out.selected_items);

    let mut tables: Vec<TableReference> = Vec::new();
    for item in selected_items.unwrap().iter() {
        let table = TableReference::from_str(format!("{}", item.output()).as_str());
        match table {
            Ok(table) => tables.push(table),
            Err(e) => eprintln!("Error parsing table reference: {}", e),
        }
    }

    tables
}
