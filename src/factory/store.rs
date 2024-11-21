use crate::{Error, Props};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

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

    ///
    ///
    ///
    pub fn into_json_value(&self) -> Result<JsonValue, Error> {
        serde_json::to_value(&self.products).map_err(|e| Error::InternalLogic(format!("{:?}", e)))
    }
}
