mod btree;


use std::path::Path;

use btree::{
    node::Node, error::Error
};

use crate::btree::{node_type::{NodeType, KeyValuePair}, page::Page, btree::BTreeBuilder};



fn main () {
    let mut btree = BTreeBuilder::new()
    .path(Path::new("/tmp/db"))
    .b_parameter(2)
    .build().unwrap();

// Write some data.
    btree.insert(KeyValuePair::new("a".to_string(), "shalom".to_string())).unwrap();
    btree.insert(KeyValuePair::new("b".to_string(), "hello".to_string())).unwrap();
    btree.insert(KeyValuePair::new("c".to_string(), "marhaba".to_string())).unwrap();
}