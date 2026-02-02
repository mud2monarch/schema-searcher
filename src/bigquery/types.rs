use serde::{Deserialize, Serialize};

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

// Defining serde structs for GET(tables.list) response.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableList {
    pub tables: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    #[serde(rename = "tableReference")]
    pub table_reference: TableReference,
    #[serde(skip)]
    pub schema: Option<Vec<Column>>,
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

// Defining serde structs for GET(tables.get) response.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableGet {
    pub schema: Schema,
}

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
