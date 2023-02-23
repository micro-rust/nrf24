//! CRC configuration.



#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CRCBytes {
    /// One byte used for CRC.
    OneByte,

    /// Two bytes used for CRC.
    TwoBytes,
}
