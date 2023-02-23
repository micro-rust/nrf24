//! Data Rate of the device.



#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DataRate {
    /// 250 kbps data rate.
    Low,

    /// 1 Mbps data rate.
    High,

    /// 2 Mbps data rate.
    Max,
}
