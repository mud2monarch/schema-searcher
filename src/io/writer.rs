use crate::bigquery::types::Column;
use std::fs::File;
use std::io::Write;

fn write_column(file: &mut File, column: &Column, indent: usize) -> std::io::Result<()> {
    let indent_str = " ".repeat(indent);
    writeln!(
        file,
        "{}|- {} ({}) [{}]",
        indent_str,
        column.name,
        column.field_type,
        column.mode.as_deref().unwrap_or("REQUIRED")
    )?;

    // If this is a RECORD type, print its nested fields
    if let Some(fields) = &column.fields {
        for field in fields {
            write_column(file, field, indent + 2)?; // Increase indentation for nested fields
        }
    }

    Ok(())
}
