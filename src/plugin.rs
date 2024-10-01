use std::ffi::{c_char, CString};

///
/// This structure provides the plugin interface
///
/// It means that all the plugins have to provide this structure
///
#[repr(C)]
pub struct Plugin {
    pub name: *const c_char,
    pub version: *const c_char,
}

impl Plugin {
    pub fn new(name: &str, version: &str) -> Self {
        let name_cstr = CString::new(name).unwrap();
        let version_cstr = CString::new(version).unwrap();

        Plugin {
            name: name_cstr.as_ptr(),
            version: version_cstr.as_ptr(),
        }
    }
}
