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

impl IndexFieldOrder {
    fn string_value(&self) -> &str {
        match &self {
            IndexFieldOrder::ASC => "1",
            IndexFieldOrder::DSC => "-1",
        }
    }
}

impl IndexSchema {
    fn new(fields: Vec<IndexFieldSchema>, is_unique: bool) -> Self {
        let name = fields
            .iter()
            .map(|field| {
                format!(
                    "{}_{}",
                    field.field_name,
                    field.order.string_value()
                )
            })
            .collect::<Vec<_>>()
            .join("_");

        IndexSchema {
            is_unique,
            fields,
            name
        }
    }
}