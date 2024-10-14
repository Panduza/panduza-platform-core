use std::ffi::CString;

use super::Notification;

pub struct NotificationGroup {
    elements: Vec<Notification>,

    pulled_elements: Vec<Notification>,

    ///
    /// Keep it here to maintain pointer validity
    ///
    pulled_elements_serialized: CString,
}

impl NotificationGroup {
    ///
    ///
    ///
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            pulled_elements: Vec::new(),
            pulled_elements_serialized: CString::default(),
        }
    }

    ///
    ///
    ///
    pub fn push(&mut self, n: Notification) {
        self.elements.push(n);
    }

    ///
    ///
    ///
    pub fn pull_and_serialize(&mut self) -> *const i8 {
        self.pulled_elements = self.elements.clone();
        self.elements.clear();
        let json_str = serde_json::to_string(&self.pulled_elements)
            .expect("Failed to serialize pulled_elements to JSON");
        let as_c_string = CString::new(json_str);
        match as_c_string {
            Ok(o) => {
                self.pulled_elements_serialized = o;
            }
            Err(e) => {
                println!("Failed to serialize pulled_elements to JSON {:?}", e);
                self.pulled_elements_serialized = CString::default();
            }
        }
        self.pulled_elements_serialized.as_c_str().as_ptr()
    }
}
