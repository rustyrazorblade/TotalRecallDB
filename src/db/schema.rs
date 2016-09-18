use std::collections::HashMap();

// owned by a Stream
#[derive(Debug)]
struct Schema {
    num_fields: u8,
    // maps a name to an id
    fields: HashMap<String, u8>,
}