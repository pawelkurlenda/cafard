#[derive(Debug)]
pub struct IndexSchema {
    pub name: String,
    is_unique: bool,
    fields: Box<[IndexFieldSchema]>
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
    fn to_string(&self) -> &str {
        match &self {
            IndexFieldOrder::ASC => "1",
            IndexFieldOrder::DSC => "-1",
        }
    }
}

impl IndexSchema {
    pub fn new(fields: Vec<IndexFieldSchema>, is_unique: bool) -> Self {
        let name = fields
            .iter()
            .map(|field| {
                format!(
                    "{}_{}",
                    field.field_name,
                    field.order.to_string()
                )
            })
            .collect::<Vec<_>>()
            .join("_");

        let boxed_fields = fields.into_boxed_slice();

        IndexSchema {
            fields: boxed_fields,
            is_unique,
            name
        }
    }
}