//! Power State of NRF24L01(+) devices.



#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PowerState {
    /// The device is powered down.
    PowerDown,

    /// The device is in standby.
    Standby,
}
