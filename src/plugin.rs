use std::{ffi::{c_char, CString}, str::FromStr};

///
/// This structure provides the plugin interface
///
/// It means that all the plugins have to provide this structure
///
#[repr(C)]
pub struct Plugin {
    pub name: String ,
    // pub version: *const c_char,
    pub test: extern "C" fn()
}

impl Plugin {
    pub fn new(name: &str, version: &str, test: extern "C" fn()) -> Self {
        

        Plugin {
            name: String::from_str(name).unwrap(),
            // version: version.as_ptr(),
            test: test
        }
    }
}
