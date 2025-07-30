use std::fmt::{Debug, Display};
use::std::error::Error;

#[derive(Debug)]
pub struct AttributeConversionException;

impl Display for AttributeConversionException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Attribute could not be converted to its data cont_type")
    }
}

impl Error for AttributeConversionException {}