use std::collections::HashMap;

use crate::Props;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]

///
///
///
pub struct Product {
    ///
    ///
    ///
    pub description: String,

    ///
    ///
    ///
    pub props: Props,
}

#[derive(Default, Debug, Serialize, Deserialize)]
/// # Store
///
/// Contains information about drivers that can be produced by a factory
///
/// ## Json Structure
///
/// {
///     "dref" {
///         "description": "....",
///         "props": {}
///     }
/// }
///
pub struct Store {
    ///
    ///
    ///
    pub products: HashMap<String, Product>,
}

impl Store {
    ///
    /// Check if the store contains the given product ref
    ///
    pub fn contains(&self, r#ref: &String) -> bool {
        self.products.contains_key(r#ref)
    }
}
