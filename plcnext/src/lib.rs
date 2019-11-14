// Include plcnext services
mod error;

use error::Result;
use error::PlcnextError;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_ulong;
use std::slice;
use std::sync::{Once, ONCE_INIT};

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

static mut CALLBACK: Option<Box<FnMut(PlcOperation)>> = None;

// The callback function for the plcnext-sys crate
extern fn handle_event(operation: plcnext_sys::PlcOperation) {
    // Pass the operation straight through to our client
    // TODO: Use the num_enum crate to convert the primitive into our enum
    // The Box around callback is immutable so we can't borrow it for the match.
    // Instead, we must get a mutable reference using as_mut().
    // We could also write the compare as:
    //     if let Some(ref mut cb) = callback
    unsafe { if let Some(cb) = CALLBACK.as_mut() {
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
        CALLBACK = match handler {
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
            let _result = plcnext_sys::ArpSystemModule_Load(raw_arp_binary_dir, raw_application_name, raw_acf_settings_path);

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
    let mut _success : bool = false;
    let mut out_id : [u8; 32] = [0 ;32];
    unsafe {
        _success = plcnext_sys::ArpPlcDevice_GetUniqueHardwareId(out_id.as_mut_ptr());
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
            let _error = get_last_error();
        }

        // Try to release the GDS buffer before returning
        release_gds_buffer(gds_buffer).ok();  // TODO: Handle errors properly.
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
        release_gds_buffer(gds_buffer).ok();  // TODO: Handle errors properly.
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
        release_gds_buffer(gds_buffer).ok();  // TODO: Handle errors properly.
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
        release_gds_buffer(gds_buffer).ok();  // TODO: Handle errors properly.
        return Err(PlcnextError::new(&error));
    }

    // Get the offset to the named port in the GDS buffer
    let mut offset: usize = 0;
    if !unsafe {plcnext_sys::ArpPlcGds_GetVariableOffset(gds_buffer, raw_port_name, &mut offset)} {
        // Log::Error("ArpPlcGds_GetVariableOffset failed");
        // Find out what the problem was
        let error = get_last_error();
        // Try to release the GDS buffer before returning
        release_gds_buffer(gds_buffer).ok();  // TODO: Handle errors properly.
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
