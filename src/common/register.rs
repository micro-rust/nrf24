//! Registers of the NRF24L01(+) devices.




#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    /// Configuration register.
    Config = 0x00,

    /// Enable Auto-Acknowledge register.
    AutoAck = 0x01,

    /// Enable RX Address register.
    RXEnable = 0x02,

    /// Setup Address Width register.
    AddressWidth = 0x03,

    /// Setup Retries register.
    Retries = 0x04,

    /// Radio Frequency Channel register.
    RFChannel = 0x05,

    /// Radio Frequency Setup register.
    RFSetup = 0x06,

    /// Status register.
    Status = 0x07,

    /// Observe TX register.
    Observe = 0x08,

    /// Receive Power Detector / Carrier Detect register.
    Detector = 0x09,

    /// Pipe 2 Address LSB register.
    RX2Address = 0x0C,

    /// Pipe 3 Address LSB register.
    RX3Address = 0x0D,

    /// Pipe 4 Address LSB register.
    RX4Address = 0x0E,

    /// Pipe 5 Address LSB register.
    RX5Address = 0x0F,

    /// Pipe 0 Payload Width register.
    RX0Width = 0x11,

    /// Pipe 1 Payload Width register.
    RX1Width = 0x12,

    /// Pipe 2 Payload Width register.
    RX2Width = 0x13,

    /// Pipe 3 Payload Width register.
    RX3Width = 0x14,

    /// Pipe 4 Payload Width register.
    RX4Width = 0x15,

    /// Pipe 5 Payload Width register.
    RX5Width = 0x16,

    /// FIFO Status register.
    FifoStatus = 0x17,

    /// Dynamic Payload register.
    DynamicPayload = 0x1C,

    /// Feature register.
    Feature = 0x1D,
}



#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum BlockRegister {
    /// Pipe 0 RX Address register.
    RX0Address = 0x0A,

    /// Pipe 1 RX Address register.
    RX1Address = 0x0B,

    /// TX Address register.
    TXAddress = 0x10,
}
