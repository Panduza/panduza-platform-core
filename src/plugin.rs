pub mod macro_helper;
use std::ffi::CStr;

///
/// !!!!!
/// Increment this number after a Plugin structure modification
/// !!!!!
///
static C_INTERFACE_VERSION: u32 = 0;

///
/// This structure provides the plugin interface
///
/// It means that all the plugins have to provide this structure
///
#[repr(C)]
pub struct Plugin {
    ///
    /// Version of this structure which is the interface
    /// between plugins and platform
    ///
    pub c_interface_version: u32,

    ///
    ///
    pub name: *const i8,
    pub version: *const i8,

    ///
    /// Must be called to join the plugin thread
    ///
    pub join: unsafe extern "C" fn(),

    ///
    /// Return the list of all references managed by this plugin
    ///
    /// The returned list must be a json array of string
    ///
    pub producer_refs: unsafe extern "C" fn() -> *const i8,

    ///
    /// Produce a device matching the given json string configuration
    ///
    pub produce: unsafe extern "C" fn(*const i8) -> u32,

    ///
    /// Return the notifications
    ///
    pub pull_notifications: unsafe extern "C" fn() -> *const i8,
}

impl Plugin {
    pub fn new(
        name: &'static CStr,
        version: &CStr,
        join: unsafe extern "C" fn(),
        producer_refs: unsafe extern "C" fn() -> *const i8,
        produce: unsafe extern "C" fn(*const i8) -> u32,
        pull_notifications: unsafe extern "C" fn() -> *const i8,
    ) -> Self {
        Plugin {
            c_interface_version: C_INTERFACE_VERSION,
            name: name.as_ptr() as *const i8,
            version: version.as_ptr() as *const i8,
            join: join,
            producer_refs: producer_refs,
            produce: produce,
            pull_notifications: pull_notifications,
        }
    }

    // /// Converts the ProductionOrder into a C string
    // ///
    // /// Don't forget "".as_c_str().as_ptr()" to use it with the DLL interfaces
    // ///
    // pub fn to_c_string(&self) -> Result<CString, crate::Error> {
    //     let json_str =
    //         serde_json::to_string(self).expect("Failed to serialize ProductionOrder to JSON");
    //     CString::new(json_str)
    //         .map_err(|e| crate::Error::InternalLogic(format!("Failed to build CString ({:?})", e)))
    // }

    ///
    /// Converts a C-style string pointer into a `ProductionOrder`
    ///
    pub unsafe fn producer_refs_as_obj(&self) -> Result<Vec<String>, crate::Error> {
        let c_str = (self.producer_refs)();

        //
        //
        if c_str.is_null() {
            return Err(crate::Error::InvalidArgument(
                "Null C string pointer".to_string(),
            ));
        }

        //
        //
        let c_str = unsafe { CStr::from_ptr(c_str) };
        let str = c_str
            .to_str()
            .map_err(|e| crate::Error::InvalidArgument(format!("Invalid C string: {:?}", e)))?;

        println!("{:?}", str);

        let json: serde_json::Value = serde_json::from_str(str)
            .map_err(|e| crate::Error::InvalidArgument(format!("Invalid JSON: {:?}", e)))?;

        let obj = serde_json::from_value(json).map_err(|e| {
            crate::Error::InvalidArgument(format!("Failed to deserialize JSON: {:?}", e))
        })?;

        Ok(obj)
    }
}
