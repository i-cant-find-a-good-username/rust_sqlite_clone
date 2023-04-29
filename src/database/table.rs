use std::collections::{HashMap, HashSet};

enum DataType {
    Integer,
    String,
    Float,
    Boolean,
    Invalid,
}

struct Table {
    name: String,
    columns: Vec<Column>,
    last_id: u64,
    primary_key: String,
}

struct Row {
    name: String,
    tables: HashMap<String, String>,
}

struct Column {
    name: String,
    data_type: DataType,
    is_pk: bool,
    is_unique: bool,
    nullable: bool,
}
