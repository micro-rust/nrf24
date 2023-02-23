//! Commands of NRF24L01(+) devices.


#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Command {
    /// Flushes the TX FIFO.
    TXFlush = 0b1110_0001,

    /// Flushes the RX FIFO.
    RXFlush = 0b1110_0010,

    /// Sends the previous payload.
    TXReusePayload = 0b1110_0011,

    /// Reads the RX payload width.
    RXPayloadWidth = 0b0110_0000,

    /// Reads the RX payload.
    RXPayload = 0b0110_0001,

    /// No operation.
    Nop = 0b1111_1111,
}
