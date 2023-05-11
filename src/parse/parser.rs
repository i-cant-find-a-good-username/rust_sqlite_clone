use crate::commands::sql_command;
use super::tokenizer;

pub enum Statement {
    Query(Box<Query>),
    pub struct Query {
        /// WITH (common table expressions, or CTEs)
        pub with: Option<With>,
        /// SELECT or UNION / EXCEPT / INTERSECT
        pub body: SetExpr,
        /// ORDER BY
        pub order_by: Vec<OrderByExpr>,
        /// `LIMIT { <N> | ALL }`
        pub limit: Option<Expr>,
        /// `OFFSET <N> [ { ROW | ROWS } ]`
        pub offset: Option<Offset>,
        /// `FETCH { FIRST | NEXT } <N> [ PERCENT ] { ROW | ROWS } | { ONLY | WITH TIES }`
        pub fetch: Option<Fetch>,
        /// `FOR { UPDATE | SHARE }`
        pub lock: Option<LockType>,
    }
    
    Insert {
        into: bool,
        table_name: ObjectName,
        columns: Vec<Ident>,
        /// A SQL query that specifies what to insert
        source: Box<Query>,
        /// partitioned insert (Hive)
        partitioned: Option<Vec<Expr>>,
        /// Columns defined after PARTITION
        after_columns: Vec<Ident>,
        /// whether the insert has the table keyword (Hive)
        table: bool,
        on: Option<OnInsert>,
    },
    Update {
        /// TABLE
        table: TableWithJoins,
        /// Column assignments
        assignments: Vec<Assignment>,
        /// Table which provide value to be set
        from: Option<TableWithJoins>,
        /// WHERE
        selection: Option<Expr>,
    },
    Delete {
        /// FROM
        table_name: ObjectName,
        /// WHERE
        selection: Option<Expr>,
    },
    CreateTable {
        name: ObjectName,
        columns: Vec<ColumnDef>,
        constraints: Vec<TableConstraint>,
        hive_distribution: HiveDistributionStyle,
        hive_formats: Option<HiveFormat>,
        table_properties: Vec<SqlOption>,
        with_options: Vec<SqlOption>,
        file_format: Option<FileFormat>,
        location: Option<String>,
        query: Option<Box<Query>>,
        without_rowid: bool,
        like: Option<ObjectName>,
        engine: Option<String>,
        default_charset: Option<String>,
        collation: Option<String>,
        on_commit: Option<OnCommit>,
    },
    Drop {
        object_type: ObjectType,
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