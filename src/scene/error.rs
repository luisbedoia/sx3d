use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Sx3dError {
    pub message: String,
    pub error: Option<Box<dyn Error>>,
}

impl fmt::Display for Sx3dError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!")
    }
}

impl Sx3dError {
    pub fn get_error_message(&self) -> String {
        String::from(&self.message)
    }
}
