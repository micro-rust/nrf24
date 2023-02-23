//! Address width (in bytes) used in communication.



#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AddressWidth {
    /// Address width of 3 bytes.
    ThreeBytes,

    /// Address width of 4 bytes.
    FourBytes,

    /// Address width of 5 bytes.
    FiveBytes,
}
