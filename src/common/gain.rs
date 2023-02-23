//! Gain of the antenna.



#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Gain {
    /// Minimum gain (0 dB).
    Min,

    /// Low gain (0 dB).
    Low,

    /// High gain (0 dB).
    High,

    /// Maximum gain (0 dB).
    Max,
}
