use serde::{Deserialize, Serialize};
use serde_json::json;

///
/// Notification about interface creation
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceNotification {
    ///
    /// Interface topic with name as last layer
    ///
    pub topic: String,
    ///
    /// Interfaces tags
    ///
    pub tags: Vec<String>,
}

impl InterfaceNotification {
    pub fn new<N: Into<String>>(topic: N, tags: Vec<String>) -> Self {
        Self {
            topic: topic.into(),
            tags,
        }
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        //
        // let mut children = serde_json::Map::new();
        // for e in &self.elements {
        //     children.insert(e.name().clone(), e.into_json_value());
        // }

        return json!({
            "tags": self.tags,
            // "children": children
        });
    }

    // pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
    //     // TODO Control layers == 0

    //     // if layers.len() == 1 {
    //     //     let name = layers.get(0).ok_or(Error::Wtf)?;
    //     //     for element in &self.elements {
    //     //         if element.name() == name {
    //     //             return Ok(true);
    //     //         }
    //     //     }
    //     //     return Ok(false);
    //     // } else {
    //     //     let name = layers.get(0).ok_or(Error::Wtf)?;
    //     //     let sublayer = self.find_layer(&name);

    //     //     let mut new_la = layers;
    //     //     new_la.remove(0);
    //     //     return sublayer.is_element_exist(new_la);
    //     // }
    // }
}
