use std::collections::HashMap;

pub enum PropType {
    String,
}

///
///
///
pub struct Prop {
    description: String,
    r#type: PropType,
    default: serde_json::Value,
}

pub struct Props {
    ///
    ///
    ///
    data: HashMap<String, Prop>,
}
