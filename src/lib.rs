
extern crate xplm;
use xplm::ipc::Plugin;

use std::ffi::CString;

/// The signature of the DataRefEditor plugin
const DRE_SIGNATURE: &'static str = "xplanesdk.examples.DataRefEditor";

/// A message sent to DataRefEditor to register a dataref
const MSG_ADD_DATAREF: i32 = 0x01000000;

/// Shares a dataref with DataRefEditor
///
/// This function will have no effect if the DataRefEditor plugin is not enabled
/// and fully initialized. This typically means that it must be called after plugins
/// are enabled.
///
/// Returns an error if DataRefEditor could not be found or if the name contains a null byte.
pub fn share_dataref(name: &str) -> Result<(), Box<std::error::Error>> {
    let dre = try!(Plugin::with_signature(DRE_SIGNATURE).ok_or(NotFoundError));
    let name_c = try!(CString::new(name));
    // Send a message, with a pointer to the string as a parameter
    // As long as messages are synchronous, this should be safe.
    try!(dre.send_message(MSG_ADD_DATAREF, name_c.as_ptr() as usize));
    Ok(())
}

/// An error that indicates that the DataRefEditor plugin was not found
#[derive(Debug)]
pub struct NotFoundError;

impl ::std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "DataRefEditor plugin not found")
    }
}

impl ::std::error::Error for NotFoundError {
    fn description(&self) -> &str {
        "DataRefEditor plugin not found"
    }
}
