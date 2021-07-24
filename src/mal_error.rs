use std::fmt;

pub struct MalError {
    code: usize,
    message: String
}

impl MalError {
    pub fn new(code: usize, message: String) -> MalError {
        MalError {
            code,
            message
        }
    }
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "MalError {{ code: {}, message: {} }}",
               self.code,
               self.message
        )
    }
}

impl fmt::Debug for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MalError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }    
}
