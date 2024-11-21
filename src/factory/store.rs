use std::collections::HashMap;

use crate::Props;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]

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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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

    ///
    /// Extend the current store by copying an other
    ///
    pub fn extend_by_copy(&mut self, other: &Store) {
        self.products.extend(other.products.clone());
    }
}
