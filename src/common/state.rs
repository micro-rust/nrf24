//! State of the device.



#[derive(Clone, Copy, Eq, PartialEq)]
pub enum State {
    /// The device is powered down.
    /// In this state, it is consuming the least amount of power but cannot transmit or receive.
    PowerDown,

    /// The device is powered up.
    /// In this state the device is ready to transmit or receive data.
    Standby,

    /// The device is actively listening for data.
    /// In this state the device is listening for data in any of its pipes.
    Listening,

    /// The device is actively transmitting data.
    Transmitting,
}
