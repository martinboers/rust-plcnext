//
// Copyright (c) 2019 Phoenix Contact GmbH & Co. KG. All rights reserved.
// Licensed under the MIT. See LICENSE file in the project root for full license information.
//
// Error messages are from the following Phoenix Contact user manual:
// "Axioline F: Diagnostic registers, and error messages"
//    Designation: UM EN AXL F SYS DIAG
//    Revision: 03
//    Date: 11 November 2016

use std::error;
use std::fmt;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Define a structure for the error texts
#[derive(Debug)]
struct Message {
    error: &'static str,
    info: String,
    remedy: &'static str,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// Store static texts in a lookup table
lazy_static! {
    // Error messages
    static ref ERROR: HashMap<u16, &'static str> = {
        let mut map = HashMap::new();

        // User errors
        map.insert(0x0903, "Memory problem (e.g., buffer too small).");
        map.insert(0x0904, "Inconsistent parameters.");
        map.insert(0x0905, "Invalid parameters.");
        map.insert(0x0908, "Maximum number of permitted parallel services exceeded (Processing conflict).");
        map.insert(0x090A, "The number of parameters is inconsistent with the service. \
                            The Parameter_Count parameter does not agree with the number of subsequent words.");
        map.insert(0x0913, "The service called is not supported.");
        map.insert(0x0917, "Service decoding failed.");
        map.insert(0x0918, "Call of an unknown service code.");
        map.insert(0x0928, "An exclusive service was to be executed without the appropriate rights.");
        map.insert(0x0932, "Attempt to pass on the exclusive rights without having these rights.");
        map.insert(0x0933, "Another node has currently the exclusive rights.");
        map.insert(0x0934, "The node already has the exclusive rights.");
        map.insert(0x0937, "Unknown Variable_ID component.");
        map.insert(0x0938, "An internal Variable_ID was used.");
        map.insert(0x0939, "The Variable_ID is not enabled. (Password protection).");
        map.insert(0x093A, "Length specification in the Variable_ID is 0 or incorrect.");
        map.insert(0x093B, "The number of variables has been calculated incorrectly.");

        map.insert(0x0A01, "A hardware or firmware error occurred.");
        map.insert(0x0A02, "A service was called that is not permitted in the current status of the local bus master.");
        map.insert(0x0A03, "Memory problem (e.g., buffer too small).");
        map.insert(0x0A04, "Inconsistent parameters.");
        map.insert(0x0A05, "Invalid parameters.");
        map.insert(0x0A06, "Access not supported.");
        map.insert(0x0A07, "Object does not exist.");
        map.insert(0x0A08, "Maximum number of permitted parallel SM services exceeded. (Processing conflict).");
        map.insert(0x0A0C, "Call of Set_Value or Read_Value with a Variable_ID that contains an unknown code.");
        map.insert(0x0A0D, "A firmware error occurred.");
        map.insert(0x0A18, "A reserved bit is set in Used_Attributes.");
        map.insert(0x0A19, "The end of the frame was exceeded when accessing the configuration or line 0 was accessed.");
        map.insert(0x0A1A, "The frame reference specified for the service does not exist.");
        map.insert(0x0A1C, "Maximum number of devices exceeded.");
        map.insert(0x0A2F, "Number of devices is zero.");
        map.insert(0x0A51, "A frame reference from 1 to 254 is permitted only.");
        map.insert(0x0A54, "The maximum number of I/O points was exceeded.");
        map.insert(0x0A60, "No configuration frames could be assigned.");
        map.insert(0x0A70, "A reserved bit has been set in the Diag_Info attribute.");
        map.insert(0x0A73, "Device present with a chip version in the local bus that is not supported.");
        map.insert(0x0A74, "Device of a manufacturer that is not supported present in the local bus.");
        map.insert(0x0A75, "Device is indicating a serious error (e. g., faulty EEPROM).");
        map.insert(0x0A76, "The topology used by the device is not supported by the master.");
        map.insert(0x0A77, "Error at the interface.");
        map.insert(0x0A7A, "Invalid Dev_Type specified during loading.");
        map.insert(0x0A7B, "Invalid Dev_ID specified during loading.");
        map.insert(0x0A7C, "Invalid Dev_Length specified during loading.");
        map.insert(0x0A81, "Service (e.g, Create_Configuration) could not be executed \
                            due to PDI communication malfunctions (timeout).");
        map.insert(0x0A82, "Service (e.g, Create_Configuration) could not be executed \
                            due to PDI communication malfunctions (number).");
        map.insert(0x0A83, "Service (e.g, Create_Configuration) could not be executed \
                            due to PDI communication malfunctions (error).");
        map.insert(0x0A90, "Device was selected for synchronization, \
                            however it does not support this.");
        map.insert(0x0A91, "Device was selected for synchronization, \
                            however it does not support the specified cycle time.");
        map.insert(0x0A92, "Device was selected for synchronization, \
                            but does not support the specified value for Input_Delay.");
        map.insert(0x0A93, "Device was selected for synchronization, \
                            but does not support the specified value for Output_Delay.");
        map.insert(0x0A94, "Device was selected for synchronization, \
                            but does not support the specified values for Input_Delay and Output_Delay.");
        map.insert(0x0AFF, "Call of Reset_Driver during PDI communication.");

        map.insert(0x0B01, "A hardware or firmware error occurred.");
        map.insert(0x0B02, "A hardware or firmware error occurred.");
        map.insert(0x0B03, "A hardware or firmware error occurred.");
        map.insert(0x0B04, "A hardware or firmware error occurred.");
        map.insert(0x0B05, "Invalid parameters.");
        map.insert(0x0B06, "Access not supported. (E.g., write protection).");
        map.insert(0x0B07, "Object does not exist.");
        map.insert(0x0B0C, "A hardware or firmware error occurred.");
        map.insert(0x0BC1, "Supply voltage not available for the local bus. Too many devices connected \
                            or the higher-level power supply unit is too weak.");
        map.insert(0x0BDE, "Synchronization failed. Trigger signal does not correspond to the specification.");

        // Bus diagnostics
        map.insert(0x0BD1, "The bus could not be activated due to bus malfunctions.");
        map.insert(0x0BF1, "The bus could not be activated due to bus malfunctions.");
        map.insert(0x0BF2, "The bus could not be activated due to bus malfunctions.");
        map.insert(0x0BF3, "The bus could not be activated due to bus malfunctions.");

        map.insert(0x0C01, "The configured module is not accessible. \
                            A device present in the configuration frame has been removed from the \
                            physical bus structure after the configuration frame has been connected.");
        map.insert(0x0C02, "A module has been detected that was not configured. \
                            An additional device was added at the end of the \
                            physical bus structure after the configuration frame was connected.");
        map.insert(0x0C11, "The module is not located in the configured slot. \
                            An active device was inserted at the different location of the physical \
                            bus structure after the configuration frame was connected.");
        map.insert(0x0C12, "The module is accessible but was not put into operation due to missing parameters. \
                            An active device was replaced by an unknown device in the physical \
                            bus structure after the configuration frame was connected (wrong instance ID).");
        map.insert(0x0C13, "The process data length does not correspond to the configured value. \
                            The process data width of an active device was changed after the \
                            configuration frame was connected.");
        map.insert(0x0C14, "The module type does not correspond to the configured value.");
        map.insert(0x0C15, "The module ID does not correspond to the configured value.");


            // PDI service
        map.insert(0x0201, "Unable to access the object. Possible causes: \
                            (a) Module not present, (b) Incorrect module number.");
        map.insert(0x0200, "Error in the communication relationship.");

        map.insert(0x0501, "The current object state prevents the service from being executed.");
        map.insert(0x0502, "Problem with the PDU size and/or permissible length exceeded. \
                            Object cannot be read completely.");
        map.insert(0x0503, "The service cannot be executed at present.");
        map.insert(0x0504, "The service contains inconsistent parameters.");
        map.insert(0x0505, "A parameter has an invalid value.");
        map.insert(0x0500, "Faulty service.");

        map.insert(0x0601, "Invalid object.");
        map.insert(0x0602, "Hardware fault.");
        map.insert(0x0603, "Access to object denied.");
        map.insert(0x0604, "Access to an invalid address.");
        map.insert(0x0605, "Inconsistent object attribute.");
        map.insert(0x0606, "The service used cannot be applied to this object.");
        map.insert(0x0607, "Object does not exist.");
        map.insert(0x0608, "Type conflict.");
        map.insert(0x060A, "Data not ready at present.");
        map.insert(0x0600, "Faulty access.");

        map.insert(0x0800, "A reserved bit or reserved code was used during parameterization.");
        map.insert(0x0801, "Error reading or writing the object.");

        map.insert(0x0F01, "Hardware or firmware error.");
        map.insert(0x0F02, "Hardware or firmware error.");
        map.insert(0x0F03, "Hardware or firmware error.");
        map.insert(0x0F04, "Inconsistent parameters.");
        map.insert(0x0F05, "Invalid parameters.");
        map.insert(0x0F06, "Access not supported.");
        map.insert(0x0F08, "Maximum number of permitted parallel PDI services exceeded.");
        map.insert(0x0F0C, "Incorrect variable ID for Set_Value or Read_Value.");
        map.insert(0x0F0D, "Internal error.");
        map.insert(0x0F11, "Device not accessible (bus error).");
        map.insert(0x0F12, "Device cannot be reached (timeout).");
        map.insert(0x0F13, "Device not accessible because it was removed.");
        map.insert(0x0F21, "Invalid slot number (Value is 0 or larger than the maximum number of devices).");
        map.insert(0x0F22, "Slot is not active.");
        map.insert(0x0F23, "Invalid data length.");
        map.insert(0x0F24, "Invalid number of parameters.");
        map.insert(0x0F0D | 0x0F31 | 0x0F32 | 0x0F33, "Internal error.");
        map.insert(0x0F0D | 0x0F31 | 0x0F32 | 0x0F33, "Internal error.");
        map.insert(0x0F0D | 0x0F31 | 0x0F32 | 0x0F33, "Internal error.");

        map
    };

    // Remedy messages
    static ref REMEDY: HashMap<u16, &'static str> = {
        let mut map = HashMap::new();

        // User errors
        map.insert(0x0903, "Reduce the amount of data.");
        map.insert(0x0904, "Check the parameters.");
        map.insert(0x0905, "Check the parameters.");
        map.insert(0x0908, "Wait for the service called previously to be completed, and then try again.");
        map.insert(0x090A, "Match the number of parameters.");
        map.insert(0x0913, "Use a service that is supported.");
        map.insert(0x0917, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0918, "Check the call.");
        map.insert(0x0928, "Wait for the exclusive rights to be enabled.");
        map.insert(0x0933, "Wait for the exclusive rights to be enabled.");
        map.insert(0x0937, "Check the call.");
        map.insert(0x0938, "Check the call.");
        map.insert(0x0939, "Check the call.");
        map.insert(0x093A, "Check the call.");
        map.insert(0x093B, "Check the call.");

        map.insert(0x0A01, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0A02, "Set the local bus master to the required state.");
        map.insert(0x0A03, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0A04, "Check the call.");
        map.insert(0x0A05, "Check the call.");
        map.insert(0x0A06, "Check the call.");
        map.insert(0x0A07, "Check the call.");
        map.insert(0x0A08, "Wait for the service called previously to be completed, and then try again.");
        map.insert(0x0A0C, "Check the call.");
        map.insert(0x0A0D, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0A18, "Check the parameters.");
        map.insert(0x0A19, "Check the access.");
        map.insert(0x0A1A, "Check the parameters.");
        map.insert(0x0A1C, "Reduce the bus configuration.");
        map.insert(0x0A2F, "Connect the device and check the connection.");
        map.insert(0x0A51, "Currently, the value 1 is permitted only.");
        map.insert(0x0A54, "Reduce the number of I/O points to the maximum number. To obtain the exact number, \
                            please refer to the documentation for your controller.");
        map.insert(0x0A60, "Create the configuration frame.");
        map.insert(0x0A70, "Check the parameters.");
        map.insert(0x0A73, "Replace the device.");
        map.insert(0x0A74, "Replace the device.");
        map.insert(0x0A75, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0A76, "Replace the device.");
        map.insert(0x0A77, "Check the connection between the electronics module and bus base module.");
        map.insert(0x0A7A, "Check the parameters.");
        map.insert(0x0A7B, "Check the parameters.");
        map.insert(0x0A7C, "Check the parameters.");
        map.insert(0x0A81, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0A82, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0A83, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0A90, "Select a device that supports synchronization or change the selection.");
        map.insert(0x0A91, "Select a different cycle time or a different device.");
        map.insert(0x0A92, "Select a different value for Input_Delay or a different device.");
        map.insert(0x0A93, "Select a different value for Output_Delay or a different device.");
        map.insert(0x0A94, "Selected different values for Input_Delay and Output_Delay or a different device.");
        map.insert(0x0AFF, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");

        map.insert(0x0B01, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0B02, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0B03, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0B04, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0B05, "Check the parameters.");
        map.insert(0x0B06, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0B07, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0B0C, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0BC1, "Use a suitable power supply unit. Check the power consumption of the devices; \
                            if required, use a power module for communications power or install a further Axioline F station.");
        map.insert(0x0BDE, "Check the synchronization signal of the higher-level system. \
                            Make sure that the cycle time specification is properly selected.");

        // Bus diagnostics
        map.insert(0x0BD1, "Check the bus configuration.");
        map.insert(0x0BF1, "Check the bus configuration.");
        map.insert(0x0BF2, "Check the bus configuration.");
        map.insert(0x0BF3, "Check the bus configuration.");

        map.insert(0x0C01, "Check the configuration. Adapt the configuration frame if the modification was done on purpose.");
        map.insert(0x0C02, "Check the configuration. Adapt the configuration frame if the modification was done on purpose.");
        map.insert(0x0C11, "Check the configuration. Adapt the configuration frame if the modification was done on purpose.");
        map.insert(0x0C12, "Check the configuration. Adapt the configuration frame if the modification was done on purpose.");
        map.insert(0x0C13, "Check the configuration. Adapt the configuration frame if the modification was done on purpose.");
        map.insert(0x0C14, "Check the configuration. Adapt the configuration frame if the modification was done on purpose.");
        map.insert(0x0C15, "Check the configuration. Adapt the configuration frame if the modification was done on purpose.");

        // PDI service
        map.insert(0x0201, "Check the call.");
        map.insert(0x0200, "Check the call.");

        map.insert(0x0501, "Check the call.");
        map.insert(0x0502, "Check the call.");
        map.insert(0x0503, "Check the call.");
        map.insert(0x0504, "Check the call.");
        map.insert(0x0505, "Check the call.");
        map.insert(0x0500, "Check the call.");

        map.insert(0x0601, "Check the call.");
        map.insert(0x0602, "Eliminate the hardware error (e.g., I/O voltage not present). \
                            Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0603, "Check the call.");
        map.insert(0x0604, "Check the call.");

        map.insert(0x0800, "Check the parameterization.");
        map.insert(0x0801, "Check the call.");

        map.insert(0x0F01, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0F02, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0F03, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0F04, "Check the parameters.");
        map.insert(0x0F05, "Check the parameters.");
        map.insert(0x0F06, "Check the call.");
        map.insert(0x0F08, "Wait until the services have been processed.");
        map.insert(0x0F0C, "Check the call.");
        map.insert(0x0F0D, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0F11, "Check the bus configuration.");
        map.insert(0x0F12, "Check the device.");
        map.insert(0x0F13, "Check the bus configuration.");
        map.insert(0x0F21, "Check the call.");
        map.insert(0x0F22, "Check the call.");
        map.insert(0x0F23, "Check the call.");
        map.insert(0x0F24, "Check the call.");
        map.insert(0x0F31, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0F32, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");
        map.insert(0x0F33, "Restart the device. If the problem still occurs, please contact Phoenix Contact.");

        map
    };

    // Additional info
    static ref INFO: HashMap<u16, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0x0000, "No detailed information on the cause of error.");
        map.insert(0x0010, "Service parameter with invalid value.");
        map.insert(0x0011, "Subindex not available.");
        map.insert(0x0012, "Object access is not a request.");
        map.insert(0x0013, "Service code is not supported.");
        map.insert(0x0014, "Subslot is not supported.");
        map.insert(0x0015, "Object access type not supported on this object.");
        map.insert(0x0016, "Object access request index for this AccessType does not equal 0x0000.");
        map.insert(0x0017, "Object access request length for this AccessType does not equal zero.");
        map.insert(0x0018, "Object length for this object does not match.");
        map.insert(0x0019, "Object is ReadOnly and cannot be overwritten.");
        map.insert(0x001A, "Object is WriteOnly and cannot be read.");
        map.insert(0x001B, "Write/read access to the object is not permitted.");
        map.insert(0x001C, "Access requires Upload-Read or Download-Write.");
        map.insert(0x0020, "Service cannot be executed at present.");
        map.insert(0x0021, "Due to local control, service cannot be executed at present.");
        map.insert(0x0022, "Service cannot be executed in current device state (device control).");
        map.insert(0x0023, "Service cannot be executed at present as no object dictionary is available.");
        map.insert(0x0030, "Value range of a parameter out of range.");
        map.insert(0x0031, "Parameter value too large.");
        map.insert(0x0032, "Parameter value too small.");
        map.insert(0x0040, "Collision with other values.");
        map.insert(0x0041, "Communication object cannot be mapped to the process data.");
        map.insert(0x0042, "Process data length exceeded.");
        map.insert(0x0050, "Firmware download rejected: general.");
        map.insert(0x0051, "Firmware download rejected: incorrect update version.");
        map.insert(0x0052, "Firmware download rejected: incorrect firmware version for the hardware.");
        map.insert(0x0053, "Firmware download rejected: identical firmware block.");
        map.insert(0x0080, "Hardware error.");
        map.insert(0x0081, "Application failed.");
        map.insert(0x00A0, "Invalid segment number, e.g., upload without initiation with subindex == 0xFF.");
        map.insert(0x00A1, "Resource not available; No more resources (memory) available for download.");
        map.insert(0x00A2, "Incorrect CRC (checksum).");
        map.insert(0x00A3, "Error opening the file (if file system is available).");
        map.insert(0x00A4, "Error writing the file (if file system is available).");
        map.insert(0x00A5, "Error closing the file (if file system is available).");
        map.insert(0x00A6, "Segment missing: Fewer data blocks were received than specified in the last segment.");
        map.insert(0x00A7, "Excess segment: More data blocks were received than specified in the last segment.");
        map.insert(0x00A8, "Error reading the file (if file system is available).");
        map.insert(0x00A9, "Segment number invalid or duplicated (segment ignored).");
        map.insert(0x00B1, "The password cannot be replaced (deleted).");
        map.insert(0x00B2, "The password cannot be added (too many passwords).");
        map.insert(0x00B3, "The password cannot be assigned for the desired type of access.");

        map
    };
}

pub type Result<T> = std::result::Result<T, AxiolineError>;

#[derive(Debug)]
pub struct AxiolineError {
    pub error_code: u16,
    pub add_info: u16,
}

impl AxiolineError {
    pub fn new(error_code: u16, add_info: u16) -> AxiolineError {
        AxiolineError{ error_code, add_info }
    }
}

impl fmt::Display for AxiolineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msb:u8 = ((&self.error_code & 0xFF00) >> 8) as u8;
        let lsb:u8 = (&self.error_code & 0x00FF) as u8;
        let info = &self.add_info;

        // Since there may be several messages for one error,
        // use a vector to store messages and then join them at the end.
        let mut error: Vec<String> = vec![];     // Error text
        let mut add_info_used = false;  // Indicates that add_info contains an error-specific value

        let blah = match &self.error_code {
            // Error codes for user errors and bus diagnostics
            0x0908 => format!("Code of failed service: 0x{0:04X}", info),
            //     0x0A => error.push(String::from("The number of parameters is inconsistent with the service. \
            //                         The Parameter_Count parameter does not agree with the number of subsequent words. \
            //                         'add_info' contains value transmitted in Parameter_Count.")),
            //     0x13 => error.push(String::from("The service called is not supported. \
            //                         'add_info' contains code of failed service.")),
            //     0x17 => error.push(String::from("Service decoding failed. Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact.
            //                         'add_info' contains code of failed service.")),
            //     0x18 => error.push(String::from("Call of an unknown service code. \
            //                         'add_info' contains code of the unknown service.")),
            //     0x28 => error.push(String::from("An exclusive service was to be executed without the appropriate rights. \
            //                         Wait for the exclusive rights to be enabled.")),
            //     0x32 => error.push(String::from("Attempt to pass on the exclusive rights without having these rights.")),
            //     0x33 => error.push(String::from("Another node has currently the exclusive rights. \
            //                         Wait for the exclusive rights to be enabled.")),
            //     0x34 => error.push(String::from("The node already has the exclusive rights.")),
            //     0x37 => error.push(String::from("Unknown Variable_ID component. \
            //                         'add_info' contains faulty Variable_ID.")),
            //     0x38 => error.push(String::from("An internal Variable_ID was used. \
            //                         'add_info' contains reserved Variable_ID.")),
            //     0x39 => error.push(String::from("The Variable_ID is not enabled. (Password protection). \
            //                         'add_info' contains variable_ID not enabled.")),
            //     0x3A => error.push(String::from("Length specification in the Variable_ID is 0 or incorrect. \
            //                         'add_info' contains incorrect Variable_ID.")),
            //     0x3B => error.push(String::from("The number of variables has been calculated incorrectly. \
            //                         'add_info' contains incorrect Variable_Count.")),
            //     _ => ()
            // },

            // 0x0A => match lsb {
            //     0x01 => error.push(String::from("A hardware or firmware error occurred. Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact.")),
            //     0x02 => error.push(String::from("A service was called that is not permitted in the current status of the local bus master. \
            //                         Set the local bus master to the required state. \
            //                         'add_info' contains current status of the local bus master.")),
            //     0x03 => error.push(String::from("Memory problem (e.g., buffer too small). Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact.")),
            //     0x04 => error.push(String::from("Inconsistent parameters.")),
            //     0x05 => error.push(String::from("Invalid parameters.")),
            //     0x06 => error.push(String::from("Access not supported.")),
            //     0x07 => error.push(String::from("Object does not exist.")),
            //     0x08 => error.push(String::from("Maximum number of permitted parallel SM services exceeded. (Processing conflict). \
            //                         Wait for the service called previously to be completed, and then try again. \
            //                         'add_info' contains code of failed service.")),
            //     0x0C => error.push(String::from("Call of Set_Value or Read_Value with a Variable_ID that contains an unknown code. \
            //                         'add_info' contains unknown Variable_ID.")),
            //     0x0D => error.push(String::from("A firmware error occurred. Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact.")),
            //     0x18 => error.push(String::from("A reserved bit is set in Used_Attributes. \
            //                         'add_info' contains invalid Used_Attributes parameter.")),
            //     0x19 => error.push(String::from("The end of the frame was exceeded when accessing the configuration or line 0 was accessed. \
            //                         'add_info' contains number of bus devices.")),
            //     0x1A => error.push(String::from("The frame reference specified for the service does not exist. \
            //                         'add_info' contains invalid Frame_Reference (if specified).")),
            //     0x1C => error.push(String::from("Maximum number of devices exceeded. \
            //                         'add_info' contains number of connected devices.")),
            //     0x2F => error.push(String::from("Number of devices is zero. Connect the device and check the connection.")),
            //     0x51 => error.push(String::from("A frame reference from 1 to 254 is permitted only. \
            //                         Currently, the value 1 is permitted only.")),
            //     0x54 => error.push(String::from("The maximum number of I/O points was exceeded. \
            //                         Reduce the number of I/O points to the maximum number. \
            //                         To obtain the exact number, please refer to the documentation for your controller.")),
            //     0x60 => error.push(String::from("No configuration frames could be assigned. Create the configuration frame.")),
            //     0x70 => error.push(String::from("A reserved bit has been set in the Diag_Info attribute.")),
            //     0x73 => error.push(String::from("Device present with a chip version in the local bus that is not supported. \
            //                         'add_info' contains device number.")),
            //     0x74 => error.push(String::from("Device of a manufacturer that is not supported present in the local bus. \
            //                         'add_info' contains device number.")),
            //     0x75 => error.push(String::from("Device is indicating a serious error (e. g., faulty EEPROM). Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact. \
            //                         'add_info' contains device number.")),
            //     0x76 => error.push(String::from("The topology used by the device is not supported by the master. Replace the device. \
            //                         'add_info' contains device number.")),
            //     0x77 => error.push(String::from("Error at the interface. Check the connection between the electronics module and bus base module. \
            //                         'add_info' contains device number.")),
            //     0x7A => error.push(String::from("Invalid Dev_Type specified during loading.")),
            //     0x7B => error.push(String::from("Invalid Dev_ID specified during loading.")),
            //     0x7C => error.push(String::from("Invalid Dev_Length specified during loading.")),
            //     0x81 => error.push(String::from("Service (e.g, Create_Configuration) could not be executed \
            //                         due to PDI communication malfunctions (timeout). Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact. \
            //                         'add_info' contains object index.")),
            //     0x82 => error.push(String::from("Service (e.g, Create_Configuration) could not be executed \
            //                         due to PDI communication malfunctions (number). Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact. \
            //                         'add_info' contains object index.")),
            //     0x83 => error.push(String::from("Service (e.g, Create_Configuration) could not be executed \
            //                         due to PDI communication malfunctions (error). Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact. \
            //                         'add_info' contains object index.")),
            //     0x90 => error.push(String::from("Device was selected for synchronization, however it does not support this. \
            //                         Select a device that supports synchronization or change the selection. \
            //                         'add_info' contains device number.")),
            //     0x91 => error.push(String::from("Device was selected for synchronization, however it does not support the specified cycle time. \
            //                         Select a different cycle time or a different device. \
            //                         'add_info' contains device number.")),
            //     0x92 => error.push(String::from("Device was selected for synchronization, but does not support the specified value for Input_Delay. \
            //                         Select a different value for Input_Delay or a different device. \
            //                         'add_info' contains device number.")),
            //     0x93 => error.push(String::from("Device was selected for synchronization, but does not support the specified value for Output_Delay. \
            //                         Select a different value for Output_Delay or a different device. \
            //                         'add_info' contains device number.")),
            //     0x94 => error.push(String::from("Device was selected for synchronization, but does not support the \
            //                         specified values for Input_Delay and Output_Delay. \
            //                         Selected different values for Input_Delay and Output_Delay or a different device. \
            //                         'add_info' contains device number.")),
            //     0xFF => error.push(String::from("Call of Reset_Driver during PDI communication. Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact.")),
            //     _ => ()
            // },

            // 0x0B => match lsb {
            //     0x01 | 0x02 | 0x03 | 0x04 | 0x0C => error.push(String::from("A hardware or firmware error occurred. Restart the device. \
            //                                              If the problem still occurs, please contact Phoenix Contact.")),
            //     0x05 => error.push(String::from("Invalid parameters.")),
            //     0x06 => error.push(String::from("Access not supported. (E.g., write protection). Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact.")),
            //     0x07 => error.push(String::from("Object does not exist. Restart the device. \
            //                         If the problem still occurs, please contact Phoenix Contact.")),
            //     0xC1 => error.push(String::from("Supply voltage not available for the local bus. Too many devices connected \
            //                         or the higher-level power supply unit is too weak. \
            //                         Use a suitable power supply unit. \
            //                         Check the power consumption of the devices; if required, use a power module \
            //                         for communications power or install a further Axioline F station.")),
            //     0xDE => error.push(String::from("Synchronization failed. Trigger signal does not correspond to the specification. \
            //                         Check the synchronization signal of the higher-level system. \
            //                         Make sure that the cycle time specification is properly selected.")),
            //     0xD1 | 0xF1 | 0xF2 | 0xF3 => error.push(String::from("The bus could not be activated due to bus malfunctions.")),
            //     _ => ()
            // },

            // 0x0C => match lsb {
            //     0x01 => error.push(String::from("The configured module is not accessible. \
            //                         A device present in the configuration frame has been removed from the \
            //                         physical bus structure after the configuration frame has been connected. \
            //                         Adapt the configuration frame if the modification was done on purpose.
            //                         'add_info' contains device number.")),
            //     0x02 => error.push(String::from("A module has been detected that was not configured. \
            //                         An additional device was added at the end of the \
            //                         physical bus structure after the configuration frame was connected. \
            //                         Adapt the configuration frame if the modification was done on purpose.
            //                         'add_info' contains device number.")),
            //     0x11 => error.push(String::from("The module is not located in the configured slot. \
            //                         An active device was inserted at the different location of the physical \
            //                         bus structure after the configuration frame was connected. \
            //                         Adapt the configuration frame if the modification was done on purpose.
            //                         'add_info' contains device number.")),
            //     0x12 => error.push(String::from("The module is accessible but was not put into operation due to missing parameters. \
            //                         An active device was replaced by an unknown device in the physical \
            //                         bus structure after the configuration frame was connected (wrong instance ID). \
            //                         Adapt the configuration frame if the modification was done on purpose.
            //                         'add_info' contains device number.")),
            //     0x13 => error.push(String::from("The process data length does not correspond to the configured value. \
            //                         The process data width of an active device was changed after the \
            //                         configuration frame was connected. \
            //                         Adapt the configuration frame if the modification was done on purpose.
            //                         'add_info' contains device number.")),
            //     0x14 => error.push(String::from("The module type does not correspond to the configured value. \
            //                         Adapt the configuration frame if the modification was done on purpose.
            //                         'add_info' contains device number.")),
            //     0x15 => error.push(String::from("The module ID does not correspond to the configured value. \
            //                         Adapt the configuration frame if the modification was done on purpose.
            //                         'add_info' contains device number.")),
            //     _ => ()
            // },

            // // Error codes when calling the PDI services
            // 0x02 => { error.push(String::from("Error in the communication relationship."));
            //     match lsb {
            //         0x01 => error.push(String::from("Unable to access the object. Possible causes: \
            //                             (a) Module not present, (b) Incorrect module number.")),
            //         _ => ()
            //     };
            // },
            // 0x05 => { error.push(String::from("Faulty Service."));
            //     match lsb {
            //         0x01 => error.push(String::from("The current object state prevents the service from being executed.")),
            //         0x02 => error.push(String::from("Problem with the PDU size and/or permissible length exceeded. \
            //                             Object cannot be read completely.")),
            //         0x03 => error.push(String::from("The service cannot be executed at present.")),
            //         0x04 => error.push(String::from("The service contains inconsistent parameters.")),
            //         0x05 => error.push(String::from("A parameter has an invalid value.")),
            //         _ => ()
            //     };
            // },
            // 0x06 => { error.push(String::from("Faulty access."));
            //     match lsb {
            //         0x01 => error.push(String::from("Invalid object.")),
            //         0x02 => error.push(String::from("Hardware fault.  Eliminate the hardware error \
            //                             (e.g., I/O voltage not present). Restart the device. \
            //                             If the problem still occurs, please contact Phoenix Contact.")),
            //         0x03 => error.push(String::from("Access to object denied.")),
            //         0x04 => error.push(String::from("Access to an invalid address.")),
            //         0x05 => error.push(String::from("Inconsistent object attribute.")),
            //         0x06 => error.push(String::from("The service used cannot be applied to this object.")),
            //         0x07 => error.push(String::from("Object does not exist.")),
            //         0x08 => error.push(String::from("Type conflict.")),
            //         0x0A => error.push(String::from("Data not ready at present.")),
            //         _ => ()
            //     };
            // },
            // 0x08 => match lsb {
            //     0x00 => { error.push(String::from("A reserved bit or reserved code was used during parameterization."));
            //         match (info & 0x00FF) as u8 {
            //             0x30 => {
            //                 error.push(String::from("Most significant byte of 'add_info' contains the number of the affected elements."));
            //                 add_info_used = true;
            //             },
            //             _ => ()
            //         };
            //     },
            //     0x01 => error.push(String::from("Error reading or writing the object.")),
            //     _ => ()
            // },
            // 0x0F => match lsb {
            //     0x01 | 0x02 | 0x03 => error.push(String::from("Hardware or firmware error. Restart the device. \
            //                                       If the problem still occurs, please contact Phoenix Contact.")),
            //     0x04 => error.push(String::from("Inconsistent parameters.")),
            //     0x05 => { 
            //         error.push(String::from("Invalid parameters. 'add_info' contains PDI object index."));
            //         add_info_used = true;
            //     },
            //     0x06 => {
            //         error.push(String::from("Access not supported. 'add_info' contains PDI object index."));
            //         add_info_used = true;
            //     },
            //     0x08 => {
            //         error.push(String::from("Maximum number of permitted parallel PDI services exceeded. \
            //                     'add_info' contains PDI object index. \
            //                     Wait until the services have been processed."));
            //         add_info_used = true;
            //     },
            //     0x0C => {
            //         error.push(String::from("Incorrect variable ID for Set_Value or Read_Value. 'add_info' contains the unknown Variable_ID."));
            //         add_info_used = true;
            //     },
            //     0x0D | 0x31 | 0x32 | 0x33 => error.push(String::from("Internal error. Restart the device. \
            //                                              If the problem still occurs, please contact Phoenix Contact.")),
            //     0x11 => error.push(String::from("Device not accessible (bus error). Check the bus configuration.")),
            //     0x12 => error.push(String::from("Device cannot be reached (timeout). Check the device.")),
            //     0x13 => error.push(String::from("Device not accessible because it was removed. Check the bus configuration.")),
            //     0x21 => {
            //         error.push(String::from("Invalid slot number (Value is 0 or larger than the maximum number of devices). \
            //                     'add_info' contains the invalid device number."));
            //         add_info_used = true;
            //     },
            //     0x22 => {
            //         error.push(String::from("Slot is not active. 'add_info' contains the invalid device number."));
            //         add_info_used = true;
            //     },
            //     0x23 => {
            //         error.push(String::from("Invalid data length. 'add_info' contains the invalid data length."));
            //         add_info_used = true;
            //     },
            //     0x24 => {
            //         error.push(String::from("Invalid number of parameters. 'add_info' contains the invalid number of parameters."));
            //         add_info_used = true;
            //     },
            //     _ => ()
            _ => String::from("")
        };

        // Check if we need to add text related to add_info
        if !add_info_used {
            if let Some(text) = INFO.get(info) {
                error.push(text.to_string());
            }
        }

        // Get error text
        let error = match ERROR.get(&self.error_code) {
            Some(text) => text,
            None => "",
        };

        // Get remedy text
        let remedy = match REMEDY.get(&self.error_code) {
            Some(text) => text,
            None => "",
        };

        // Get additional info text


        // Construct the message
            let my_error = Message {
                error: error,
                info: String::from("hello"),
                remedy: remedy
            };

        write!(f, "{} {}", self, my_error)


    }
}

impl error::Error for AxiolineError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
