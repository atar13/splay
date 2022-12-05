use std::error::Error;
use std::fmt;

//Include more details about cause of ImportError
pub enum ImportError {
    MissingData,
    FileNotFound,
    Parsing,
}

impl Error for ImportError {}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingData => {
                return write!(f, "Could not gather parts of metadata.");
            }
            Self::FileNotFound => {
                return write!(f, "file not found");
            }
            Self::Parsing => {
                return write!(f, "Parsing error");
            }
        }
    }
}

impl fmt::Debug for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingData => {
                return write!(f, "Could not gather parts of metadata.");
            }
            Self::FileNotFound => {
                return write!(f, "file not found");
            }
            Self::Parsing => {
                return write!(f, "Parsing error");
            }
        }
    }
}
