pub struct Node {
    pub node_type: NodeType,
    pub is_root: bool,
    pub parent_offset: Option<Offset>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NodeType {
    Internal(Vec<Offset>, Vec<Key>),
    
    Leaf(Vec<KeyValuePair>),
    
    Unexpected,
}














































/// Implement TryFrom<Page> for Node allowing for easier
/// deserialization of data from a Page.
impl TryFrom<Page> for Node {
    type Error = Error;
    fn try_from(page: Page) -> Result<Node, Error> {
        let raw = page.get_data();
        let node_type = NodeType::from(raw[NODE_TYPE_OFFSET]);
        let is_root = raw[IS_ROOT_OFFSET].from_byte();
        let parent_offset: Option<Offset>;
        if is_root {
            parent_offset = None;
        } else {
            parent_offset = Some(Offset(page.get_value_from_offset(PARENT_POINTER_OFFSET)?));
        }

        match node_type {
            NodeType::Internal(mut children, mut keys) => {
                let num_children = page.get_value_from_offset(INTERNAL_NODE_NUM_CHILDREN_OFFSET)?;
                let mut offset = INTERNAL_NODE_HEADER_SIZE;
                for _i in 1..=num_children {
                    let child_offset = page.get_value_from_offset(offset)?;
                    children.push(Offset(child_offset));
                    offset += PTR_SIZE;
                }

                // Number of keys is always one less than the number of children (i.e. branching factor)
                for _i in 1..num_children {
                    let key_raw = page.get_ptr_from_offset(offset, KEY_SIZE);
                    let key = match str::from_utf8(key_raw) {
                        Ok(key) => key,
                        Err(_) => return Err(Error::UTF8Error),
                    };
                    offset += KEY_SIZE;
                    // Trim leading or trailing zeros.
                    keys.push(Key(key.trim_matches(char::from(0)).to_string()));
                }
                Ok(Node::new(
                    NodeType::Internal(children, keys),
                    is_root,
                    parent_offset,
                ))
            }

            NodeType::Leaf(mut pairs) => {
                let mut offset = LEAF_NODE_NUM_PAIRS_OFFSET;
                let num_keys_val_pairs = page.get_value_from_offset(offset)?;
                offset = LEAF_NODE_HEADER_SIZE;

                for _i in 0..num_keys_val_pairs {
                    let key_raw = page.get_ptr_from_offset(offset, KEY_SIZE);
                    let key = match str::from_utf8(key_raw) {
                        Ok(key) => key,
                        Err(_) => return Err(Error::UTF8Error),
                    };
                    offset += KEY_SIZE;

                    let value_raw = page.get_ptr_from_offset(offset, VALUE_SIZE);
                    let value = match str::from_utf8(value_raw) {
                        Ok(val) => val,
                        Err(_) => return Err(Error::UTF8Error),
                    };
                    offset += VALUE_SIZE;

                    // Trim leading or trailing zeros.
                    pairs.push(KeyValuePair::new(
                        key.trim_matches(char::from(0)).to_string(),
                        value.trim_matches(char::from(0)).to_string(),
                    ))
                }
                Ok(Node::new(NodeType::Leaf(pairs), is_root, parent_offset))
            }

            NodeType::Unexpected => Err(Error::UnexpectedError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::node::{
        Node, Page, INTERNAL_NODE_HEADER_SIZE, KEY_SIZE, LEAF_NODE_HEADER_SIZE, PTR_SIZE,
        VALUE_SIZE,
    };
    use crate::node_type::{Key, NodeType};
    use crate::page_layout::PAGE_SIZE;
    use std::convert::TryFrom;

    #[test]
    fn page_to_node_works_for_leaf_node() -> Result<(), Error> {
        const DATA_LEN: usize = LEAF_NODE_HEADER_SIZE + KEY_SIZE + VALUE_SIZE;
        let page_data: [u8; DATA_LEN] = [
            0x01, // Is-Root byte.
            0x02, // Leaf Node type byte.
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Parent offset.
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // Number of Key-Value pairs.
            0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00, // "hello"
            0x77, 0x6f, 0x72, 0x6c, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, // "world"
        ];
        let junk: [u8; PAGE_SIZE - DATA_LEN] = [0x00; PAGE_SIZE - DATA_LEN];
        let mut page = [0x00; PAGE_SIZE];
        for (to, from) in page.iter_mut().zip(page_data.iter().chain(junk.iter())) {
            *to = *from
        }

        let node = Node::try_from(Page::new(page))?;

        assert_eq!(node.is_root, true);
        Ok(())
    }

    #[test]
    fn page_to_node_works_for_internal_node() -> Result<(), Error> {
        use crate::node_type::Key;
        const DATA_LEN: usize = INTERNAL_NODE_HEADER_SIZE + 3 * PTR_SIZE + 2 * KEY_SIZE;
        let page_data: [u8; DATA_LEN] = [
            0x01, // Is-Root byte.
            0x01, // Internal Node type byte.
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Parent offset.
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // Number of children.
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, // 4096  (2nd Page)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, // 8192  (3rd Page)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, // 12288 (4th Page)
            0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00, // "hello"
            0x77, 0x6f, 0x72, 0x6c, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, // "world"
        ];
        let junk: [u8; PAGE_SIZE - DATA_LEN] = [0x00; PAGE_SIZE - DATA_LEN];

        // Concatenate the two arrays; page_data and junk.
        let mut page = [0x00; PAGE_SIZE];
        for (to, from) in page.iter_mut().zip(page_data.iter().chain(junk.iter())) {
            *to = *from
        }

        let node = Node::try_from(Page::new(page))?;

        if let NodeType::Internal(_, keys) = node.node_type {
            assert_eq!(keys.len(), 2);

            let Key(first_key) = match keys.get(0) {
                Some(key) => key,
                None => return Err(Error::UnexpectedError),
            };
            assert_eq!(first_key, "hello");

            let Key(second_key) = match keys.get(1) {
                Some(key) => key,
                None => return Err(Error::UnexpectedError),
            };
            assert_eq!(second_key, "world");
            return Ok(());
        }

        Err(Error::UnexpectedError)
    }
}











































































/// Page is a wrapper for a single page of memory
/// providing some helpful helpers for quick access.
pub struct Page {
    data: Box<[u8; PAGE_SIZE]>,
}


/// Implement TryFrom<Box<Node>> for Page allowing for easier
/// serialization of data from a Node to an on-disk formatted page.
impl TryFrom<&Node> for Page {
    type Error = Error;
    fn try_from(node: &Node) -> Result<Page, Error> {
        let mut data: [u8; PAGE_SIZE] = [0x00; PAGE_SIZE];
        // is_root byte
        data[IS_ROOT_OFFSET] = node.is_root.to_byte();

        // node_type byte
        data[NODE_TYPE_OFFSET] = u8::from(&node.node_type);

        // parent offest
        if !node.is_root {
            match node.parent_offset {
                Some(Offset(parent_offset)) => data
                    [PARENT_POINTER_OFFSET..PARENT_POINTER_OFFSET + PARENT_POINTER_SIZE]
                    .clone_from_slice(&parent_offset.to_be_bytes()),
                // Expected an offset of an inner / leaf node.
                None => return Err(Error::UnexpectedError),
            };
        }

        match &node.node_type {
            NodeType::Internal(child_offsets, keys) => {
                data[INTERNAL_NODE_NUM_CHILDREN_OFFSET
                    ..INTERNAL_NODE_NUM_CHILDREN_OFFSET + INTERNAL_NODE_NUM_CHILDREN_SIZE]
                    .clone_from_slice(&child_offsets.len().to_be_bytes());

                let mut page_offset = INTERNAL_NODE_HEADER_SIZE;
                for Offset(child_offset) in child_offsets {
                    data[page_offset..page_offset + PTR_SIZE]
                        .clone_from_slice(&child_offset.to_be_bytes());
                    page_offset += PTR_SIZE;
                }

                for Key(key) in keys {
                    let key_bytes = key.as_bytes();
                    let mut raw_key: [u8; KEY_SIZE] = [0x00; KEY_SIZE];
                    if key_bytes.len() > KEY_SIZE {
                        return Err(Error::UnexpectedError);
                    } else {
                        for (i, byte) in key_bytes.iter().enumerate() {
                            raw_key[i] = *byte;
                        }
                    }
                    data[page_offset..page_offset + KEY_SIZE].clone_from_slice(&raw_key);
                    page_offset += KEY_SIZE
                }
            }
            NodeType::Leaf(kv_pairs) => {
                // num of pairs
                data[LEAF_NODE_NUM_PAIRS_OFFSET
                    ..LEAF_NODE_NUM_PAIRS_OFFSET + LEAF_NODE_NUM_PAIRS_SIZE]
                    .clone_from_slice(&kv_pairs.len().to_be_bytes());

                let mut page_offset = LEAF_NODE_HEADER_SIZE;
                for pair in kv_pairs {
                    let key_bytes = pair.key.as_bytes();
                    let mut raw_key: [u8; KEY_SIZE] = [0x00; KEY_SIZE];
                    if key_bytes.len() > KEY_SIZE {
                        return Err(Error::UnexpectedError);
                    } else {
                        for (i, byte) in key_bytes.iter().enumerate() {
                            raw_key[i] = *byte;
                        }
                    }
                    data[page_offset..page_offset + KEY_SIZE].clone_from_slice(&raw_key);
                    page_offset += KEY_SIZE;

                    let value_bytes = pair.value.as_bytes();
                    let mut raw_value: [u8; VALUE_SIZE] = [0x00; VALUE_SIZE];
                    if value_bytes.len() > VALUE_SIZE {
                        return Err(Error::UnexpectedError);
                    } else {
                        for (i, byte) in value_bytes.iter().enumerate() {
                            raw_value[i] = *byte;
                        }
                    }
                    data[page_offset..page_offset + VALUE_SIZE].clone_from_slice(&raw_value);
                    page_offset += VALUE_SIZE;
                }
            }
            NodeType::Unexpected => return Err(Error::UnexpectedError),
        }

        Ok(Page::new(data))
    }
}


mod tests {
    use crate::error::Error;
    use crate::node::Node;
    use crate::node_type::{KeyValuePair, NodeType};
    use crate::page::Page;
    use std::convert::TryFrom;
    #[test]
    fn node_to_page_works_for_leaf_node() -> Result<(), Error> {
        let some_leaf = Node::new(
            NodeType::Leaf(vec![
                KeyValuePair::new("foo".to_string(), "bar".to_string()),
                KeyValuePair::new("lebron".to_string(), "james".to_string()),
                KeyValuePair::new("ariana".to_string(), "grande".to_string()),
            ]),
            true,
            None,
        );

        // Serialize data.
        let page = Page::try_from(&some_leaf)?;
        // Deserialize back the page.
        let res = Node::try_from(page)?;

        assert_eq!(res.is_root, some_leaf.is_root);
        assert_eq!(res.node_type, some_leaf.node_type);
        assert_eq!(res.parent_offset, some_leaf.parent_offset);
        Ok(())
    }

     #[test]
    fn node_to_page_works_for_internal_node() -> Result<(), Error> {
        use crate::node::Node;
        use crate::node_type::{Key, NodeType, Offset};
        use crate::page::Page;
        use crate::page_layout::PAGE_SIZE;
        use std::convert::TryFrom;

        let internal_node = Node::new(
            NodeType::Internal(
                vec![
                    Offset(PAGE_SIZE),
                    Offset(PAGE_SIZE * 2),
                    Offset(PAGE_SIZE * 3),
                    Offset(PAGE_SIZE * 4),
                ],
                vec![
                    Key("foo bar".to_string()),
                    Key("lebron".to_string()),
                    Key("ariana".to_string()),
                ],
            ),
            true,
            None,
        );

        // Serialize data.
        let page = Page::try_from(&internal_node)?;
        // Deserialize back the page.
        let res = Node::try_from(page)?;

        assert_eq!(res.is_root, internal_node.is_root);
        assert_eq!(res.node_type, internal_node.node_type);
        assert_eq!(res.parent_offset, internal_node.parent_offset);
        Ok(())
    }
}