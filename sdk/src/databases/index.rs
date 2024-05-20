

#[derive(Debug)]
pub struct IndexSchema {
    name: String,
    is_unique: bool,
    fields: Vec<IndexFieldSchema>
}

#[derive(Debug)]
pub struct IndexFieldSchema {
    field_name: String,
    order: IndexFieldOrder
}

#[derive(Debug)]
pub enum IndexFieldOrder {
    ASC,
    DSC
}