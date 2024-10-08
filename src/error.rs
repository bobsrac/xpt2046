//! Error definition for the crate

#[cfg(feature = "with_defmt")]
use defmt::{write, Format, Formatter};

#[cfg_attr(feature = "with_defmt", derive(Format))]
#[derive(Debug)]
pub enum CalibrationError {
    Alpha,
    Beta,
    Delta,
}

#[derive(Debug)]
pub enum Error<SpiError> {
    /// SPI bus error
    Spi(SpiError),
    /// Error when calculating new calibration values
    Calibration(CalibrationError),
    /// Delay error
    Delay,
}

#[cfg(feature = "with_defmt")]
impl<E> Format for Error<E> {
    fn format(&self, fmt: Formatter) {
        match self {
            Error::Bus(_) => write!(fmt, "Bus error"),
            Error::Calibration(e) => write!(fmt, "Error when calculating calibration for: {}", e),
            Error::Delay => write!(fmt, "Delay error"),
        }
    }
}
