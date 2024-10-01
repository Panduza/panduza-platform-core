use std::ffi::CStr;

pub struct Builder {}

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
    pub join: extern "C" fn(),

    ///
    /// Produce a device matching the given json string configuration
    pub produce: extern "C" fn(*const i8),
    //
    //
    // get_producers -> function to get producers
    // produce -> function to create a new device
    //
}

impl Plugin {
    pub fn new(
        name: &'static CStr,
        version: &CStr,
        test: extern "C" fn(),
        join: extern "C" fn(),
        produce: extern "C" fn(*const i8),
    ) -> Self {
        Plugin {
            name: name.as_ptr() as *const i8,
            version: version.as_ptr() as *const i8,
            test: test,
            join: join,
            produce: produce,
        }
    }
}
