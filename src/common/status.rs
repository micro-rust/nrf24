//! A representation of the Status register of NRF24L01(+) devices.
//! Contains helper methods to avoid doing bitwise operations on the raw registers.



#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct Status(pub(crate) u8);

impl Status {

}
