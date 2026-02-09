# BigQuery Schema Collector

A command-line tool to fetch and document BigQuery table schemas. This tool can collect schema information for all tables in a specified project. You'll be able to pick the specific tables for which you want to collect tables.

A video on the motivations for this tool can be found [here](https://www.youtube.com/watch?v=ytdzjhRwQzY).

## Prerequisites

- You need a Google Cloud service account credentials file with BigQuery read access. The default name is `service_account_creds.json`, in the same working directory as the executable.
- The service account must have access to the BigQuery projects you want to query.

## Installation

### With executable

1. Download the executable from the release page.
2. Place your service account credentials JSON file in the same working directory as the executable (default name: `service_account_creds.json`)
3. Run the executable from the command line (see Usage section below).

### With source code

1. Clone the repository.
2. Place your service account credentials JSON file in the project directory (default name: `service_account_creds.json`)
3. Build the project:
```bash
cargo build --release
```
4. Run the executable from the command line (see Usage section below).
## Usage

### With executable
```bash
# free the executable from quarantine
xattr -d com.apple.quarantine ./schema-searcher
# make the file executable
chmod +x ./schema-searcher
# run the executable
./schema-searcher
```

You'll be prompted to enter the:
1. Path to your service account credentials JSON file
2. Project ID
3. Name for output file

### Output Format

The tool generates a text file with schema information in the following format:

```
=== Table: project.dataset.table ===
|- column_name (TYPE) [MODE]
  |- nested_field (TYPE) [MODE]  # For RECORD types
```
