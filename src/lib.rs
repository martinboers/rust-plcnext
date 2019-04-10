extern crate plcnext_sys;

use std::ffi::CStr;
use std::ffi::CString;

const MAX_ERROR_LENGTH: usize = 512;

pub fn system_module_load() -> ::std::os::raw::c_int {

    let arp_binary_dir = CString::new("/usr/lib").expect("CString::new failed");
    let application_name = CString::new("PLCnextSampleRuntime").expect("CString::new failed");
    let acf_settings_path = CString::new("/opt/plcnext/projects/Default/Default.acf.settings").expect("CString::new failed");

    // Create pointers to strings
    let raw_arp_binary_dir = arp_binary_dir.into_raw();
    let raw_application_name = application_name.into_raw();
    let raw_acf_settings_path = acf_settings_path.into_raw();

    unsafe {
        let result = plcnext_sys::ArpSystemModule_Load(raw_arp_binary_dir, raw_application_name, raw_acf_settings_path);

        // Retake pointers to free memory
        let _ = CString::from_raw(raw_arp_binary_dir);
        let _ = CString::from_raw(raw_application_name);
        let _ = CString::from_raw(raw_acf_settings_path);

        result
    }
}

// TODO: Return Result<String, Err> and handle errors
/// Copies the last error message into the buffer. After this operation the error
/// message is deleted. If there is no error message the buffer will contain only 0x00.
pub fn get_last_error() -> String {

    let mut buffer: [u8; MAX_ERROR_LENGTH] = [0x00; MAX_ERROR_LENGTH];
    unsafe {
        plcnext_sys::ArpPlc_GetLastError(buffer.as_mut_ptr(), MAX_ERROR_LENGTH as i32);
        CStr::from_bytes_with_nul_unchecked(&buffer).to_string_lossy().into_owned()
    }
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
