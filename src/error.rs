use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Err {
    StdIoErr(std::io::Error),
    GpioErr(gpio_cdev::Error),
    ParseNumErr(std::num::ParseIntError),
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::StdIoErr(_) => write!(f, "std io error"),
            Err::GpioErr(_) => write!(f, "gpio error"),
            Err::ParseNumErr(_) => write!(f, "parse number error"),
        }
    }
}

impl From<std::io::Error> for Err {
    fn from(e: std::io::Error) -> Self {
        Self::StdIoErr(e)
    }
}

impl From<gpio_cdev::Error> for Err {
    fn from(e: gpio_cdev::Error) -> Self {
        Self::GpioErr(e)
    }
}

impl From<std::num::ParseIntError> for Err {
    fn from(e: ParseIntError) -> Self {
        Self::ParseNumErr(e)
    }
}
