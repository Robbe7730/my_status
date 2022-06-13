use std::fmt::{self, Display, Formatter};
use std::sync::PoisonError;
use std::error::Error;

#[derive(Debug)]
pub struct ModuleError {
    description: String
}

impl Display for ModuleError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl<T> From<PoisonError<T>> for ModuleError {
    fn from(_error: PoisonError<T>) -> ModuleError {
        ModuleError{
            description: "Could not lock mutex".to_string()
        }
    }
}

impl Error for ModuleError {}
