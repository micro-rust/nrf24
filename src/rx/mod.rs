//! Receiver configuration of NRF24L01(+) devices.



mod config;
mod pipe;



pub use config::Config;
pub use pipe::Pipe;



use embassy_time::{
    Duration, Timer,
};

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

use super::{
    base::{
        BaseDriver,
    },
    common::{
        BlockRegister, Command, State, Register, Status,
    },
};


pub struct Receiver<SPI: SpiBus + SpiBusWrite, CS: OutputPin, CE: OutputPin, IRQ: Wait> {
    /// Basic device driver.
    pub base: BaseDriver<SPI, CS, CE, IRQ>,

    /// Configuration of the receiver.
    config: Config,

    /// Power State of the device.
    state: State,
}

impl<SPI: SpiBus + SpiBusWrite, CS: OutputPin, CE: OutputPin, IRQ: Wait> Receiver<SPI, CS, CE, IRQ> {
    /// Creates a new receiver with the given configuration.
    pub async fn new(base: BaseDriver<SPI, CS, CE, IRQ>, config: Config) -> Result<Self, SPI::Error> {
        // Configure the pipes.
        let (rxautoack, rxenable, dynpd, width, addr) = config.pipeconfig();

        // Registers to write.
        let registers = [
            (Register::Config        , config.config()   ), // 0
            (Register::AutoAck       , rxautoack         ), // 1
            (Register::RXEnable      , rxenable          ), // 2
            (Register::AddressWidth  , config.addrwidth()), // 3
            (Register::RFChannel     , config.channel    ), // 4
            (Register::RFSetup       , config.rfsetup()  ), // 5
            (Register::DynamicPayload, dynpd             ), // 6
            (Register::Feature       , config.features() ), // 7

            (Register::RX0Width      , width[0]), // 8
            (Register::RX1Width      , width[1]), // 9
            (Register::RX2Width      , width[2]), // 10
            (Register::RX3Width      , width[3]), // 11
            (Register::RX4Width      , width[4]), // 12
            (Register::RX5Width      , width[5]), // 13

            (Register::RX2Address    , addr[0]), // 14
            (Register::RX3Address    , addr[1]), // 15
            (Register::RX4Address    , addr[2]), // 16
            (Register::RX5Address    , addr[3]), // 17
        ];

        // Register block.
        let blocks = [
            (BlockRegister::RX0Address, config.address),
            (BlockRegister::RX1Address, config.secondary()),
        ];

        // Create the device.
        let mut device = Self { base, config, state: State::PowerDown, };

        // Write all the registers.
        for (r, v) in registers.iter() {
            device.base.writereg(*r, *v).await?;
        }

        // Write the two register blocks.
        for (r, data) in blocks.iter() {
            device.base.writeblock(*r, data).await?;
        }


        Ok( device )
    }

    /// Sets the RF channel used.
    pub async fn rfchannel(&mut self, channel: u8) -> Result<Status, SPI::Error> {
        // Change the channel in the config.
        self.config.channel = channel;

        // Write the change to the device.
        self.base.writereg(Register::RFChannel, channel).await
    }

    /// Powers up the device.
    pub async fn powerup(&mut self) -> Result<Status, SPI::Error> {
        // Read the register.
        let (_, register) = self.base.readreg(Register::Config).await?;

        // Write the modified register.
        self.base.writereg(Register::Config, register | (1 << 1)).await
    }

    /// Sets the device up and starts listening for incoming packets.
    pub async fn listen(&mut self) -> Result<(), SPI::Error> {
        // Power up the device.
        match self.state {
            State::PowerDown => {
                // Power up.
                self.powerup().await?;

                // Change state.
                self.state = State::Standby;
            },

            _ => (),
        }

        // If the state is standby, prepare for listening and enable CE.
        match self.state {
            State::Standby => {
                // Clear interrupts.
                self.base.writereg(Register::Status, 0b111 << 4).await?;

                // Flush the RX FIFO.
                self.base.command( Command::RXFlush ).await?;

                // Set CE high.
                self.base.enable();

                // Change state.
                self.state = State::Listening;
            },

            _ => (),
        }

        Ok(())
    }

    /// Listens for a payload in one of the active pipes.
    /// Awaits until a new packet is ready.
    pub async fn recv(&mut self, payload: Option<()>, stop: bool, timeout: Duration) -> Result<Option<Payload>, SPI::Error> {
        // Begin listening.
        self.listen().await?;

        // Wait for the IRQ.
        embassy_time::with_timeout(timeout, self.base.wait()).await;

        // Set CE low.
        self.base.disable();

        // Read the payload.
        let payload = match self.base.rxpayload().await? {
            Some((len, raw)) => {
                // Get the status.
                let status = Status(raw[0]);

                // Get the data.
                let data = <[u8; 32]>::try_from(&raw[1..]).unwrap();

                Payload {
                    len: len as usize,
                    status,
                    data,
                }
            },

            _ => return Ok( None ),
        };

        // Check if the device stops listening.
        if stop {
            self.base.disable();
        } else {
            // Clear interrupts and get ready for the next payload.
            self.base.writereg(Register::Status, 0b111 << 4).await?;
        }

        Ok( Some( payload ) )
    }

    /// Stops the device listening for packets.
    pub fn unlisten(&mut self) {
        self.base.disable();
    }
}



#[derive(Clone)]
pub struct Payload {
    /// Length of the payload.
    len: usize,

    /// Status of the device when the Payload was read.
    pub status: Status,

    /// Raw data of the payload.
    data: [u8; 32],
}

impl Payload {
    /// Returns a view into the payload.
    pub fn view(&self) -> &[u8] {
        &self.data[0..self.len]
    }
}
