use serde::{Deserialize, Serialize};

// TODO: I'm pretty sure I never need to serialize because I'm only ever reading from the BQ API.

// Defining serde structs for GET(dataset.list) response.
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetList {
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "datasetReference")]
    pub dataset_reference: DatasetReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetReference {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "datasetId")]
    pub dataset_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableList {
    pub tables: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    #[serde(rename = "tableReference")]
    pub table_reference: TableReference,
    // Note: this must be an Option because table.list returns a Table-like type that doesn't contain a schema.
    pub schema: Option<Schema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableReference {
    // In future, consider pulling partition parameter as well.
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "datasetId")]
    pub dataset_id: String,
    #[serde(rename = "tableId")]
    pub table_id: String,
}

// intermediate type that typically passes through to Column
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub mode: Option<String>,
    pub fields: Option<Vec<Column>>,
}
