
///
/// This structure provides the plugin interface
///
/// It means that all the plugins have to provide this structure
///
#[repr(C)]
pub struct Plugin {
    ///
    /// 
    pub name: *const i8,
    pub version: *const i8,
    pub test: extern "C" fn(),
    pub join: extern "C" fn()
}

impl Plugin {
    pub fn new(name: &str, version: &str, test: extern "C" fn(), join: extern "C" fn()) -> Self {
        Plugin {
            name: name.as_ptr() as *const i8,
            version: version.as_ptr() as *const i8,
            test: test,
            join: join
        }
    }
}
