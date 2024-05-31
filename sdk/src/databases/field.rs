
#[derive(Debug)]
pub struct FieldSchema {
    pub name: String,
    pub is_primary_key: bool,
    pub is_autoincrement: bool,
    pub is_row_version: bool,
    pub default_value: Option<String>,
    pub field_type: String, // todo: how to store field type (int, decimal, date, string)
    pub validators: Vec<FieldSchemaValidator>
}

#[derive(Debug)]
pub struct FieldSchemaValidator {

}