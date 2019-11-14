use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, PlcnextError>;

// TODO: Consider changing PlcnextError to be an enum of possible error types, e.g.
// - SystemError error (from the ANSI-C library)
// - Parameter error (e.g. invalid port name, data array length not correct, etc )
// - RSC Service errors? e.g. Axioline module gives its own error struct.
// Possibly define errors in each module and then wrap them here?

#[derive(Debug)]
pub struct PlcnextError {
    pub details: String
}

impl PlcnextError {
    pub fn new(msg: &str) -> PlcnextError {
        PlcnextError{ details: msg.to_string() }
    }
}

impl fmt::Display for PlcnextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl error::Error for PlcnextError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
    fn description(&self) -> &str {
        &self.details
    }
}
