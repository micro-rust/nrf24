//! Base part of the driver.
//! Contains the SPI interface and the CS, CE and IRQ pin.



#![allow(unused_must_use)]



use embedded_hal::{
    digital::v2::{
        OutputPin,
    },
};

use embedded_hal_async::{
    digital::{
        Wait,
    },
    spi::{
        SpiBus, SpiBusWrite,
    },
};

use super::common::{
    BlockRegister, Command, Register, Status,
};



pub struct BaseDriver<SPI: SpiBus + SpiBusWrite, CS: OutputPin, CE: OutputPin, IRQ: Wait> {
    /// The SPI device bus.
    spi: SPI,

    /// The Chip Select pin.
    cs: CS,

    /// The Chip Enable pin.
    ce: CE,

    /// The IRQ pin.
    irq: IRQ,
}

impl<SPI: SpiBus + SpiBusWrite, CS: OutputPin, CE: OutputPin, IRQ: Wait> BaseDriver<SPI, CS, CE, IRQ> {
    /// Write command.
    const WRITECMD: u8 = 0b00100000;

    /// Takes ownership of the resources and creates the base driver.
    pub fn new(spi: SPI, cs: CS, ce: CE, irq: IRQ) -> Self {
        Self { spi, cs, ce, irq }
    }

    /// Enables the CE signal.
    pub(crate) fn enable(&mut self) {
        self.ce.set_high();
    }

    /// Disables the CE signal.
    pub(crate) fn disable(&mut self) {
        self.ce.set_low();
    }

    /// Waits for the IRQ signal.
    pub(crate) async fn wait(&mut self) {
        self.irq.wait_for_low().await;
    }

    /// Sends a command to the device.
    pub(crate) async fn command(&mut self, cmd: Command) -> Result<Status, SPI::Error> {
        // Command to send.
        let write = [cmd as u8];

        // Input buffer.
        let mut read = [0];

        // Set CS low.
        self.cs.set_low();

        // Perform transfer.
        self.spi.transfer(&mut read, &write).await?;

        // Set CS high.
        self.cs.set_high();

        Ok( Status(read[0]) )
    }

    /// Reads the given register.
    pub async fn readreg(&mut self, r: Register) -> Result<(Status, u8), SPI::Error> {
        // Command to send.
        let write = [r as u8, 0];

        // Buffer to read.
        let mut read = [0, 0];

        // Set CS low.
        self.cs.set_low();

        // Perform the transfer.
        self.spi.transfer(&mut read, &write).await?;

        // Set CS high.
        self.cs.set_high();

        Ok( ( Status(read[0]), read[1] ) )
    }

    /// Writes to the register the given value.
    pub(crate) async fn writereg(&mut self, r: Register, v: u8) -> Result<Status, SPI::Error> {
        // Command to send.
        let write = [(r as u8) | Self::WRITECMD, v];

        // Buffer to read.
        let mut read = [0, 0];

        // Set CS low.
        self.cs.set_low();

        // Perform the transfer.
        self.spi.transfer(&mut read, &write).await?;

        // Set CS high.
        self.cs.set_high();

        Ok( Status(read[0]) )
    }

    /// Reads the given register block.
    pub async fn readblock(&mut self, r: BlockRegister) -> Result<(Status, [u8; 5]), SPI::Error> {
        // Command to send.
        let write = [(r as u8) | Self::WRITECMD, 0, 0, 0, 0, 0];

        // Input buffer.
        let mut read = [0; 6];

        // Set CS low.
        self.cs.set_low();

        // Perform the transfer.
        self.spi.transfer(&mut read, &write).await?;

        // Set CS high.
        self.cs.set_high();

        // Get status.
        let status = Status(read[0]);

        // Get data.
        // Safety: Safe because we know it will always be 5 bytes.
        let data = <[u8; 5]>::try_from(&read[1..]).unwrap();

        Ok( (status, data) )
    }

    /// Writes to the register block the given data.
    pub(crate) async fn writeblock<'a>(&mut self, r: BlockRegister, data: &'a [u8; 5]) -> Result<Status, SPI::Error> {
        // Command to send.
        let write = [
            (r as u8) | Self::WRITECMD,
            data[0],
            data[1],
            data[2],
            data[3],
            data[4],
        ];

        // Input buffer.
        let mut read = [0; 6];

        // Set CS low.
        self.cs.set_low();

        // Perform the transfer.
        self.spi.transfer(&mut read, &write).await?;

        // Set CS high.
        self.cs.set_high();

        Ok( Status(read[0]) )
    }

    /// Reads in a RX payload.
    pub(crate) async fn rxpayload(&mut self) -> Result<Option<(u8, [u8; 33])>, SPI::Error> {
        // Read the FIFO status.
        let (_, fifo) = self.readreg(Register::FifoStatus).await?;

        // If the RX is not empty, read the RX payload width.
        if (fifo & 1) != 0 {
            return Ok(None);
        }

        // Read the RX payload width.
        let len = self.rxpldwidth().await?;

        // Builds the input buffer.
        let mut words = [0; 33];
        words[0] = Command::RXPayload as u8;

        // Perform the transfer.
        self.spi.transfer_in_place(&mut words).await?;

        Ok( Some( (len, words) ) )
    }

    /// Reads the RX payload width.
    async fn rxpldwidth(&mut self) -> Result<u8, SPI::Error> {
        // Write command.
        let write = [ Command::RXPayloadWidth as u8, 0 ];

        // Input buffer.
        let mut read = [0, 0];

        // Set CS low.
        self.cs.set_low();

        // Perform the transfer.
        self.spi.transfer(&mut read, &write).await?;

        // Set CS high.
        self.cs.set_high();

        Ok( read[1] )
    }
}
