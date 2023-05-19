use super::DataType;

pub struct Column {
    name: String,
    data_type: DataType,
    pub is_pk: bool,
    pub not_null: bool,
}

pub struct CreateQuery {
    table_name: String,
    columns: Column,
}
