use super::tokenizer;

pub struct ObjectName(String);

pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub options: Vec<ColumnOptionDef>,
}

pub struct Selection {
    column: ColumnDef,
    value: DataType, // changing type
}

pub struct Assignment {}

// like not_null or auto_increment
pub struct ColumnOptionDef {
    pub name: Option<ObjectName>,
    pub option: ColumnOption,
}

pub enum DataType {
    Text(u32),    //lenght
    Integer(u32), //lenght
    Float(u32),   //lenght
    Null,
}

pub enum ObjectType {
    Table,
}

pub enum ColumnOption {
    Null,
    NotNull,
    Default(),
    Unique { is_primary: bool },
    Check(Selection),
}

pub enum Statement {
    Insert {
        into: bool,
        table_name: ObjectName,
        columns: Vec<ObjectName>,
        values: Vec<DataType>,
        table: bool,
    },
    Update {
        table: ObjectName,
        assignments: Vec<Assignment>,
        from: Option<ObjectName>,
        selection: Option<Selection>,
    },
    Delete {
        /// FROM
        table_name: ObjectName,
        /// WHERE
        selection: Option<Selection>,
    },
    CreateTable {
        name: ObjectName,
        columns: Vec<ColumnDef>,
    },
    Drop {
        object_type: ObjectName,
        names: Vec<ObjectName>,
        cascade: bool,
    },
}

pub fn parse(query: String) -> Result<Vec<tokenizer::Token>, tokenizer::TokenizerError> {
    println!("start tokenize {:?}", query);

    let mut new_tokenizer = tokenizer::Tokenizer::new(&query);
    let tokens = new_tokenizer.tokenize();

    println!("\x1b[93mtokens{:?}\x1b[0m", new_tokenizer);
    println!("\x1b[34mtokens{:?}\x1b[0m", new_tokenizer);

    tokens
    //let gg = tokenizer::tokenize();
}