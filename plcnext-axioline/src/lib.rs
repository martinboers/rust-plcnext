#![recursion_limit="128"]

// Include plcnext services
pub mod device;
pub mod io;
mod error;

use error::Result;
use error::PlcnextError;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate cpp;

use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::path::Path;
use std::os::raw::c_char;
use std::os::raw::c_ulong;
use std::slice;
use std::sync::{Once, ONCE_INIT};

use regex::Regex;

//use std::concat;

// Regular expressions for GDS port name
// Ref: PLCnext Technology User Manual
const COMPONENT_INSTANCE_RE: &str = r"(?<component>[a-zA-Z]+)([0-9a-zA-Z\-]*)([0-9a-zA-Z]+)";
const PROGRAM_INSTANCE_RE: &str = r"(?<program>[a-zA-Z]+)([0-9a-zA-Z\-]*)([0-9a-zA-Z]+)";
const PORT_RE: &str = r"(?<port>[_a-zA-Z]+)([0-9a-zA-Z\-\[\]_.]*)([0-9a-zA-Z\[\]_]+)";

const IO_NETWORK_RE: &str = r"(?<network>^(Arp.Io.AxlC)$)";  // TODO: Allow Profinet and other networks
const IO_MODULE_RE: &str = r"(?<module>^([0-9]|[1-5][0-9]|6[0-3])$)";  // TODO: What's the range on PnC modules?
const IO_PORT_RE: &str = r"(?<port>.*)";  // TODO: Fix this

lazy_static! {
    static ref PORT_NAME_RE: Regex = Regex::new(&(
        String::from(COMPONENT_INSTANCE_RE)
        + "/" + PROGRAM_INSTANCE_RE
        + "." + PORT_RE
        )).unwrap();
}

fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("login").map(|login| login.as_str())
    })
}

const MAX_ERROR_LENGTH: usize = 512;

pub enum PlcOperation {
    None = 0,
    Load = 1,
    Setup = 2,
    StartCold = 3,
    StartWarm = 4,
    StartHot = 5,
    Stop = 6,
    Reset = 7,
    Unload = 8,
    Unknown = 99
}

enum GdsOperation {
    Read,
    Write
}



// static tmol: i32 = 42;

// fn main() {
//     let myclosure = |a: PlcOperation| println!("hello {0}", tmol);
//     let myclosure2 = |a: PlcOperation| println!("hello {0}", tmol);
//     System::set_handler(Some(myclosure));
//     System::set_handler(Some(myclosure2));

//     let my_system = System::load("/usr/lib", "runtime", "/opt/plcnext/projects/runtime/runtime.acf.settings");

//     my_system.get_unique_hardware_id();

//     // Clear the handler.
//     // When passing "None", the type of the Option must be specified.
//     System::set_handler(None::<fn(_)>);
// }

// The callback pattern is based on this post:
// https://stackoverflow.com/a/41081702
// ... with the added feature that the Box<FnMut> callback
// is a std::option::Option.

// TODO: IMPLEMENT MUTEX LOCKS ON ALL UNSAFE CODE
// e.g. https://users.rust-lang.org/t/defining-a-global-mutable-structure-to-be-used-across-several-threads/7872/3
// let mut guard = SYSTEM.lock().unwrap();
//        guard.callback = Some(Box::new(callback));
// ... and check for poison mutexes.

// TODO: Consider using thread-safe smart pointer(s) so that users of this library 
// do not need to use unsafe code.

// Single system instance
// This must be mutable because the 'callback' member can be changed
// at run-time via the 'set_handler' associated function
static mut SYSTEM: System = System { s_callback: None };

static mut callback: Option<Box<FnMut(PlcOperation)>> = None;


// Another System instance cannot be created outside this crate
// because of the private member
pub struct System {
    // callback: Option<Box<FnMut(PlcOperation)>>,
    s_callback: Option<Box<FnMut(PlcOperation)>>,
}

impl System {
    // The callback function for the plcnext-sys crate
    extern fn handle_event(operation: plcnext_sys::PlcOperation) {
        // Pass the operation straight through to our client
        // TODO: Use the num_enum crate to convert the primitive into our enum
        // The Box around SYSTEM.callback is immutable so we can't borrow it for the match.
        // Instead, we must get a mutable reference using as_mut().
        // We could also write the compare as:
        //     if let Some(ref mut cb) = SYSTEM.callback
        unsafe { if let Some(s_callback) = SYSTEM.s_callback.as_mut() {
            s_callback (match operation {
                plcnext_sys::PlcOperation_PlcOperation_Load => PlcOperation::Load,
                plcnext_sys::PlcOperation_PlcOperation_Setup => PlcOperation::Setup,
                plcnext_sys::PlcOperation_PlcOperation_StartCold => PlcOperation::StartCold,
                plcnext_sys::PlcOperation_PlcOperation_StartWarm => PlcOperation::StartWarm,
                plcnext_sys::PlcOperation_PlcOperation_StartHot => PlcOperation::StartHot,
                plcnext_sys::PlcOperation_PlcOperation_Stop => PlcOperation::Stop,
                plcnext_sys::PlcOperation_PlcOperation_Reset => PlcOperation::Reset,
                plcnext_sys::PlcOperation_PlcOperation_Unload => PlcOperation::Unload,
                plcnext_sys::PlcOperation_PlcOperation_None => PlcOperation::None,
                _ => PlcOperation::Unknown
            });
            // callback(PlcOperation::Load);
    }}}

    // The handler can be set before the load function is called,
    // so that the user can receive all events as the system is loaded.
    // The user's event handler must have static lifetime because it is
    // boxed and stored in the static System instance.
    pub fn set_handler<CB: 'static + FnMut(PlcOperation)>(handler: Option<CB>) {
        // Set or reset our own callback function,
        // and also save the user's event handler.
        unsafe {
            SYSTEM.s_callback = match handler {
                Some(s_callback) => {
                    plcnext_sys::ArpPlcDomain_SetHandler(Some(System::handle_event));
                    Some(Box::new(s_callback))
                },
                None => {
                    plcnext_sys::ArpPlcDomain_SetHandler(None);
                    None
    }}}}

    // Load the ARP system module.
    // This must be called  before any system methods,
    // so this is the only way to get a reference
    // to the SYSTEM singleton.
    // Function parameters will be ignored on the second and subsequent calls, but 
    // a reference to SYSTEM will always be returned.
    pub fn load(arp_binary_dir: &str , application_name: &str, acf_settings_path: &str) -> &'static Self {
        // The system must only ever be initialised once
        static INIT: Once = ONCE_INIT;
        INIT.call_once(|| {
            // Create CStrings from inputs, for C compatibility
            let arp_binary_dir = CString::new(arp_binary_dir).expect("CString::new(arp_binary_dir) failed");
            let application_name = CString::new(application_name).expect("CString::new(application_name) failed");
            let acf_settings_path = CString::new(acf_settings_path).expect("CString::new(acf_settings_path) failed");

            // Create pointers to strings
            let raw_arp_binary_dir = arp_binary_dir.into_raw();
            let raw_application_name = application_name.into_raw();
            let raw_acf_settings_path = acf_settings_path.into_raw();

            // Call the C function
            unsafe {
                let result = plcnext_sys::ArpSystemModule_Load(raw_arp_binary_dir, raw_application_name, raw_acf_settings_path);

                // Retake pointers to free memory
                let _ = CString::from_raw(raw_arp_binary_dir);
                let _ = CString::from_raw(raw_application_name);
                let _ = CString::from_raw(raw_acf_settings_path);

                // TODO: Evaluate result and return error if necessary
                // result
            }
        });
        // Return a reference to the System instance
        unsafe { &SYSTEM }
    }

    /// Gets the unique hardware id.
    /// If an internal error occurs during the operation, an error will be returned.
    /// In this case, get the last error message with get_last_error().
    pub fn get_unique_hardware_id(&self) -> [u8; 32] {
        let mut success : bool = false;
        let mut out_id : [u8; 32] = [0 ;32];
        unsafe {
            success = plcnext_sys::ArpPlcDevice_GetUniqueHardwareId(out_id.as_mut_ptr());
        }
        // TODO: Error check on the return variable.
        out_id
    }

    /// Transfers I/O data from the Axioline bus to the GDS.
    /// * 'timeout' - Timeout in milliseconds to wait for the event to be processed.
    ///               If zero, the calling thread will block forever.
    pub fn read_from_axio_to_gds(&self, timeout: u32) -> Result<()> {
        if !unsafe { plcnext_sys::ArpPlcAxio_ReadFromAxioToGds(timeout as c_ulong) } {
            return Err(PlcnextError::new(&get_last_error()));
        }
        Ok(())
    }

    // TODO: CREATE FUNCTIONS: gds_begin(operation: read|write), gds_end(read|write)
    // Read data from a fieldbus input frame
    // TODO: Extract the fb_io_system_name from the port name.
    // TODO: Remove fb_io_system_name parameter.
    pub fn read_input_data(&self, fb_io_system_name: &str, port_name: &str, value: &[u8]) -> Result<()> {

        // TODO: Validate the port name using regex.

        // Get a pointer to the start of the GDS buffer containing the named port,
        // and the offset to the named port in that buffer
        let (gds_buffer, offset) = get_gds_port(fb_io_system_name, port_name)?;

        // ======================================
        // TODO: Replace the below with begin_gds() call
        // Begin read operation by getting a pointer to the GDS data buffer page
        // After this call, the GDS buffer will be locked
        let mut data_buffer_page: *mut c_char = std::ptr::null_mut();
        if !unsafe { plcnext_sys::ArpPlcGds_BeginRead(gds_buffer, &mut data_buffer_page) } {
            // Log::Error("ArpPlcGds_BeginRead failed");
            // Find out what the problem was
            let error = get_last_error();

            // Try to end the read operation
            if !unsafe { plcnext_sys::ArpPlcGds_EndRead(gds_buffer) } {
                // If an error occurs, just log it, but don't return it
                // Log::Error("ArpPlcGds_BeginRead failed");
                let error = get_last_error();
            }

            // Try to release the GDS buffer before returning
            release_gds_buffer(gds_buffer);
            return Err(PlcnextError::new(&error));
        }

        // Adjust the GDS buffer pointer to the port location
        let data_address = unsafe { data_buffer_page.offset(offset as isize) };

        // ======================================

        // === TODO: CONVERT THE BELOW TO READ - ITS FROM "WRITE"!
        // TODO: Copy the data from the GDS port to the input parameter.
        //
        // Construct a slice from the GDS buffer pointer, and convert it to &mut [u8]
        // TODO: MAKE SURE THE SIZE OF THE SLICE IS THE EXACT SIZE OF THE PORT!
        // If we can't do this, then we must assume that the port size is the same as the data passed in ...
        // Or, use the name of the port to guess the size of the slice.
        // let port_data = unsafe { &mut*( slice::from_raw_parts_mut(data_address, value.len()) as *mut [i8] as *mut [u8] ) };
        let port_data = unsafe { &mut*( slice::from_raw_parts_mut(data_address, value.len()) as *mut [u8] ) };

        // TODO: Copy data from the GDS Buffer
        // Copy data to the GDS Buffer
        port_data.copy_from_slice(value);

        // ======================================

        // ======================================
        // TODO: Replace the below with end_gds() call
        // Unlock and release the GDS data buffer page
        if !unsafe { plcnext_sys::ArpPlcGds_EndRead(gds_buffer) } {
            // Log::Error("ArpPlcGds_EndRead failed");
            // Find out what the problem was
            let error = get_last_error();

            // Try to release the GDS buffer before returning
            release_gds_buffer(gds_buffer);
            return Err(PlcnextError::new(&error));
        }

        // Release the GdsBuffer and free internal resources
        release_gds_buffer(gds_buffer)
        // ======================================
    }

    /// Write data to a fieldbus output frame
    // TODO: Add a check that the port exists, and return the size of the port ... then check that size against the size of the value array.
    // Include a note that the first call to this function with a new port_name is expensive, because it retrieves data from the system 
    // about that port, but after that the data is cached and read/writes are quicker.
    pub fn write_output_data(&self, fb_io_system_name: &str, port_name: &str, value: &[u8]) -> Result<()> {

        // We could consider validating the system name and port name here, but we will just pass them through to the 
        // plcnext-sys library, where (we hope) they will be validated correctly.

        // Get a pointer to the start of the GDS buffer containing the named port,
        // and the offset to the named port in that buffer
        let (gds_buffer, offset) = get_gds_port(fb_io_system_name, port_name)?;

        // Begin write operation by getting a pointer to the GDS data buffer page
        // After this call, the GDS buffer will be locked
        let mut data_buffer_page: *mut c_char = std::ptr::null_mut();
        if !unsafe { plcnext_sys::ArpPlcGds_BeginWrite(gds_buffer, &mut data_buffer_page) } {
            return Err(PlcnextError::new(&get_last_error()));
        }
        else {
            // Adjust the GDS buffer pointer to the port location
            let data_address = unsafe { data_buffer_page.offset(offset as isize) };

            // Construct a slice from the GDS buffer pointer, and convert it to &mut [u8]
            // TODO: MAKE SURE THE SIZE OF THE SLICE IS THE EXACT SIZE OF THE PORT!
            // If we can't do this, then we must assume that the port size is the same as the data passed in ...
            // let port_data = unsafe { &mut*( slice::from_raw_parts_mut(data_address, value.len()) as *mut [i8] as *mut [u8] ) };
            let port_data = unsafe { &mut*( slice::from_raw_parts_mut(data_address, value.len()) as *mut [u8] ) };

            // Copy data to the GDS Buffer
            port_data.copy_from_slice(value);
        }

        // Unlock and release the GDS data buffer page
        if !unsafe { plcnext_sys::ArpPlcGds_EndWrite(gds_buffer) } {
            let error = get_last_error();
            // Try to release the GDS buffer before returning
            // before returning the original error
            release_gds_buffer(gds_buffer);
            return Err(PlcnextError::new(&error));
        }

        // Release the GdsBuffer and free internal resources
        release_gds_buffer(gds_buffer)
    }

    /// Transfers I/O data from the GDS to the Axioline bus.
    /// * 'timeout' - Timeout in milliseconds to wait for the event to be processed.
    ///               If zero, the calling thread will block forever.
    pub fn write_from_gds_to_axio(&self, timeout: u32) -> Result<()> {
        if !unsafe { plcnext_sys::ArpPlcAxio_WriteFromGdsToAxio(timeout as c_ulong) } {
            return Err(PlcnextError::new(&get_last_error()));
        }
        Ok(())
    }
}

// The callback function for the plcnext-sys crate
extern fn handle_event(operation: plcnext_sys::PlcOperation) {
    // Pass the operation straight through to our client
    // TODO: Use the num_enum crate to convert the primitive into our enum
    // The Box around callback is immutable so we can't borrow it for the match.
    // Instead, we must get a mutable reference using as_mut().
    // We could also write the compare as:
    //     if let Some(ref mut cb) = callback
    unsafe { if let Some(cb) = callback.as_mut() {
        cb (match operation {
            plcnext_sys::PlcOperation_PlcOperation_Load => PlcOperation::Load,
            plcnext_sys::PlcOperation_PlcOperation_Setup => PlcOperation::Setup,
            plcnext_sys::PlcOperation_PlcOperation_StartCold => PlcOperation::StartCold,
            plcnext_sys::PlcOperation_PlcOperation_StartWarm => PlcOperation::StartWarm,
            plcnext_sys::PlcOperation_PlcOperation_StartHot => PlcOperation::StartHot,
            plcnext_sys::PlcOperation_PlcOperation_Stop => PlcOperation::Stop,
            plcnext_sys::PlcOperation_PlcOperation_Reset => PlcOperation::Reset,
            plcnext_sys::PlcOperation_PlcOperation_Unload => PlcOperation::Unload,
            plcnext_sys::PlcOperation_PlcOperation_None => PlcOperation::None,
            _ => PlcOperation::Unknown
        });
}}}

// The handler can be set before the load function is called,
// so that the user can receive all events as the system is loaded.
// The user's event handler must have static lifetime because it is
// boxed and stored in the static System instance.
pub fn set_handler<CB: 'static + FnMut(PlcOperation)>(handler: Option<CB>) {
    // Set or reset our own callback function,
    // and also save the user's event handler.
    unsafe {
        callback = match handler {
            Some(cb) => {
                plcnext_sys::ArpPlcDomain_SetHandler(Some(handle_event));
                Some(Box::new(cb))
            },
            None => {
                plcnext_sys::ArpPlcDomain_SetHandler(None);
                None
}}}}

// Load the ARP system module.
// This must be called  before any system methods,
// so this is the only way to get a reference
// to the SYSTEM singleton.
// Function parameters will be ignored on the second and subsequent calls, but 
// a reference to SYSTEM will always be returned.
pub fn load(arp_binary_dir: &str , application_name: &str, acf_settings_path: &str) {
    // The system must only ever be initialised once
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| {
        // Create CStrings from inputs, for C compatibility
        let arp_binary_dir = CString::new(arp_binary_dir).expect("CString::new(arp_binary_dir) failed");
        let application_name = CString::new(application_name).expect("CString::new(application_name) failed");
        let acf_settings_path = CString::new(acf_settings_path).expect("CString::new(acf_settings_path) failed");

        // Create pointers to strings
        let raw_arp_binary_dir = arp_binary_dir.into_raw();
        let raw_application_name = application_name.into_raw();
        let raw_acf_settings_path = acf_settings_path.into_raw();

        // Call the C function
        unsafe {
            let result = plcnext_sys::ArpSystemModule_Load(raw_arp_binary_dir, raw_application_name, raw_acf_settings_path);

            // Retake pointers to free memory
            let _ = CString::from_raw(raw_arp_binary_dir);
            let _ = CString::from_raw(raw_application_name);
            let _ = CString::from_raw(raw_acf_settings_path);

            // TODO: Evaluate result and return error if necessary
            // result
        }
    });
}

/// Gets the unique hardware id.
/// If an internal error occurs during the operation, an error will be returned.
/// In this case, get the last error message with get_last_error().
pub fn get_unique_hardware_id() -> [u8; 32] {
    let mut success : bool = false;
    let mut out_id : [u8; 32] = [0 ;32];
    unsafe {
        success = plcnext_sys::ArpPlcDevice_GetUniqueHardwareId(out_id.as_mut_ptr());
    }
    // TODO: Error check on the return variable.
    out_id
}

/// Transfers I/O data from the Axioline bus to the GDS.
/// * 'timeout' - Timeout in milliseconds to wait for the event to be processed.
///               If zero, the calling thread will block forever.
pub fn read_from_axio_to_gds(timeout: u32) -> Result<()> {
    if !unsafe { plcnext_sys::ArpPlcAxio_ReadFromAxioToGds(timeout as c_ulong) } {
        return Err(PlcnextError::new(&get_last_error()));
    }
    Ok(())
}

// TODO: CREATE FUNCTIONS: gds_begin(operation: read|write), gds_end(read|write)
// Read data from a fieldbus input frame
// TODO: Extract the fb_io_system_name from the port name.
// TODO: Remove fb_io_system_name parameter.
pub fn read_input_data(fb_io_system_name: &str, port_name: &str, value: &mut[u8]) -> Result<()> {

    // TODO: Validate the port name using regex.

    // Get a pointer to the start of the GDS buffer containing the named port,
    // and the offset to the named port in that buffer
    let (gds_buffer, offset) = get_gds_port(fb_io_system_name, port_name)?;

    // ======================================
    // TODO: Replace the below with begin_gds() call
    // Begin read operation by getting a pointer to the GDS data buffer page
    // After this call, the GDS buffer will be locked
    let mut data_buffer_page: *mut c_char = std::ptr::null_mut();
    if !unsafe { plcnext_sys::ArpPlcGds_BeginRead(gds_buffer, &mut data_buffer_page) } {
        // Log::Error("ArpPlcGds_BeginRead failed");
        // Find out what the problem was
        let error = get_last_error();

        // Try to end the read operation
        if !unsafe { plcnext_sys::ArpPlcGds_EndRead(gds_buffer) } {
            // If an error occurs, just log it, but don't return it
            // Log::Error("ArpPlcGds_BeginRead failed");
            let error = get_last_error();
        }

        // Try to release the GDS buffer before returning
        release_gds_buffer(gds_buffer);
        return Err(PlcnextError::new(&error));
    }

    // Adjust the GDS buffer pointer to the port location
    let data_address = unsafe { data_buffer_page.offset(offset as isize) };

    // ======================================

    // Construct a slice from the GDS buffer pointer, and convert it to &[u8]
    // TODO: MAKE SURE THE SIZE OF THE SLICE IS THE EXACT SIZE OF THE PORT!
    // If we can't do this, then we must assume that the port size is the same as the data passed in ...
    // Or, use the name of the port to guess the size of the slice.
    // let port_data = unsafe { &mut*( slice::from_raw_parts_mut(data_address, value.len()) as *mut [i8] as *mut [u8] ) };
    let port_data = unsafe { slice::from_raw_parts(data_address, value.len()) as &[u8] };

    // Copy data from the GDS Buffer
    value.copy_from_slice(port_data);

    // ======================================

    // ======================================
    // TODO: Replace the below with end_gds() call
    // Unlock and release the GDS data buffer page
    if !unsafe { plcnext_sys::ArpPlcGds_EndRead(gds_buffer) } {
        // Log::Error("ArpPlcGds_EndRead failed");
        // Find out what the problem was
        let error = get_last_error();

        // Try to release the GDS buffer before returning
        release_gds_buffer(gds_buffer);
        return Err(PlcnextError::new(&error));
    }

    // Release the GdsBuffer and free internal resources
    release_gds_buffer(gds_buffer)
    // ======================================
}

/// Write data to a fieldbus output frame
// TODO: Add a check that the port exists, and return the size of the port ... then check that size against the size of the value array.
// Include a note that the first call to this function with a new port_name is expensive, because it retrieves data from the system 
// about that port, but after that the data is cached and read/writes are quicker.
pub fn write_output_data(fb_io_system_name: &str, port_name: &str, value: &[u8]) -> Result<()> {

    // We could consider validating the system name and port name here, but we will just pass them through to the 
    // plcnext-sys library, where (we hope) they will be validated correctly.

    // Get a pointer to the start of the GDS buffer containing the named port,
    // and the offset to the named port in that buffer
    let (gds_buffer, offset) = get_gds_port(fb_io_system_name, port_name)?;

    // Begin write operation by getting a pointer to the GDS data buffer page
    // After this call, the GDS buffer will be locked
    let mut data_buffer_page: *mut c_char = std::ptr::null_mut();
    if !unsafe { plcnext_sys::ArpPlcGds_BeginWrite(gds_buffer, &mut data_buffer_page) } {
        return Err(PlcnextError::new(&get_last_error()));
    }
    else {
        // Adjust the GDS buffer pointer to the port location
        let data_address = unsafe { data_buffer_page.offset(offset as isize) };

        // Construct a slice from the GDS buffer pointer, and convert it to &mut [u8]
        // TODO: MAKE SURE THE SIZE OF THE SLICE IS THE EXACT SIZE OF THE PORT!
        // If we can't do this, then we must assume that the port size is the same as the data passed in ...
        // let port_data = unsafe { &mut*( slice::from_raw_parts_mut(data_address, value.len()) as *mut [i8] as *mut [u8] ) };
        let port_data = unsafe { &mut*( slice::from_raw_parts_mut(data_address, value.len()) as *mut [u8] ) };

        // Copy data to the GDS Buffer
        port_data.copy_from_slice(value);
    }

    // Unlock and release the GDS data buffer page
    if !unsafe { plcnext_sys::ArpPlcGds_EndWrite(gds_buffer) } {
        let error = get_last_error();
        // Try to release the GDS buffer before returning
        // before returning the original error
        release_gds_buffer(gds_buffer);
        return Err(PlcnextError::new(&error));
    }

    // Release the GdsBuffer and free internal resources
    release_gds_buffer(gds_buffer)
}

/// Transfers I/O data from the GDS to the Axioline bus.
/// * 'timeout' - Timeout in milliseconds to wait for the event to be processed.
///               If zero, the calling thread will block forever.
pub fn write_from_gds_to_axio(timeout: u32) -> Result<()> {
    if !unsafe { plcnext_sys::ArpPlcAxio_WriteFromGdsToAxio(timeout as c_ulong) } {
        return Err(PlcnextError::new(&get_last_error()));
    }
    Ok(())
}

// Gets a pointer to the GDS buffer and the offset to the port data
fn get_gds_port(fb_io_system_name: &str, port_name: &str)
    -> Result<(*mut plcnext_sys::TGdsBuffer, usize)> {

    // TODO: Consider keeping a static lookup table of bus/port & pointers to gds buffers
    // This should (?) speed up reads and writes for multiple calls using the same info.
    // (but make sure the table is thread safe)

    // Create CStrings from inputs, for C compatibility
    let fb_io_system_name = CString::new(fb_io_system_name)
        .expect("CString::new(fb_io_system_name) failed");
    let port_name = CString::new(port_name)
        .expect("CString::new(port_name) failed");

    // Create pointers to strings
    let raw_fb_io_system_name = fb_io_system_name.into_raw();
    let raw_port_name = port_name.into_raw();

    // TODO: RETAKE POINTERS

    // Create an opaque pointer to the GDS buffer
    let mut gds_buffer: *mut plcnext_sys::TGdsBuffer = std::ptr::null_mut();

    // Assign the pointer to the start of the GDS buffer containing the named port
    if !unsafe {plcnext_sys::ArpPlcIo_GetBufferPtrByPortName(raw_fb_io_system_name, raw_port_name, &mut gds_buffer)} {
        // Log::Error("ArpPlcIo_GetBufferPtrByPortName failed");
        // Find out what the problem was
        let error = get_last_error();
        // Try to release the GDS buffer before returning
        release_gds_buffer(gds_buffer);
        return Err(PlcnextError::new(&error));
    }

    // Get the offset to the named port in the GDS buffer
    let mut offset: usize = 0;
    if !unsafe {plcnext_sys::ArpPlcGds_GetVariableOffset(gds_buffer, raw_port_name, &mut offset)} {
        // Log::Error("ArpPlcGds_GetVariableOffset failed");
        // Find out what the problem was
        let error = get_last_error();
        // Try to release the GDS buffer before returning
        release_gds_buffer(gds_buffer);
        return Err(PlcnextError::new(&error));
    }

    Ok((gds_buffer, offset))
}

/// Releases the memory used by the handle to the GDS buffer
fn release_gds_buffer(gds_buffer: *mut plcnext_sys::TGdsBuffer) -> Result<()> {
    if !gds_buffer.is_null() {
        if !unsafe { plcnext_sys::ArpPlcIo_ReleaseGdsBuffer(gds_buffer) } {
            // Log::Error("ArpPlcIo_ReleaseGdsBuffer failed");
            return Err(PlcnextError::new(&get_last_error()));
        }
    }
    Ok(())
}

// TODO: Return Result<String, Err> and handle errors
// Copies the last error message into the buffer. After this operation the error
// message is deleted. If there is no error message the buffer will contain only 0x00.
pub fn get_last_error() -> String {

    let mut buffer: [u8; MAX_ERROR_LENGTH] = [0x00; MAX_ERROR_LENGTH];
    unsafe {
        plcnext_sys::ArpPlc_GetLastError(buffer.as_mut_ptr(), MAX_ERROR_LENGTH as i32);
        CStr::from_bytes_with_nul_unchecked(&buffer).to_string_lossy().into_owned()
    }
}
