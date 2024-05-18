
#[derive(Debug)]
pub struct FieldSchema {
    name: String,
    is_primary_key: bool,
    is_autoincrement: bool,
    is_row_version: bool,
    field_type: String // todo: how to store field type (int, decimal, date, string)
}