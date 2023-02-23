//! NRF24L01(+) receiver configuration.



use crate::common::{
    AddressWidth, CRCBytes,
    Gain, DataRate,
};

use super::Pipe;



#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Config {
    /// RF Channel.
    pub channel: u8,

    /// Antenna gain.
    pub gain: Gain,

    /// Device data rate.
    pub datarate: DataRate,

    /// CRC configuration.
    pub crc: Option<CRCBytes>,

    /// Address Width.
    pub addrwidth: AddressWidth,

    /// Main address.
    pub address: [u8; 5],

    /// Common sub address.
    pub subaddress: [u8; 4],

    /// Pipe configuration.
    pub pipes: [Option<Pipe>; 6],
}

impl Config {
    /// Builds the CONFIG register.
    pub(crate) const fn config(&self) -> u8 {
        // Register word.
        let mut word = 0;

        // Mask the MAX RT interrupt, set PRX.
        word |= (1 << 4) | 1;

        // Configure the CRC.
        if let Some(crc) = self.crc {
            // Set the CRC bit.
            word |= 1 << 3;

            match crc {
                CRCBytes:: TwoBytes => word |= 1 << 2,
                _ => ()
            }
        }

        word
    }

    /// Builds the RF SETUP register.
    pub(crate) const fn rfsetup(&self) -> u8 {
        // Register word.
        let mut word = 0;

        // Set the gain.
        let gain = match self.gain {
            Gain::Max  => 0b11,
            Gain::High => 0b10,
            Gain::Low  => 0b01,
            Gain::Min  => 0b00,
        };

        word |= gain << 1;

        // Set the datarate.
        match self.datarate {
            DataRate::Max => word |= 1 << 3,
            DataRate::Low => word |= 1 << 5,
            _ => (),
        }

        word
    }

    /// Creates the SETUP AW register. 
    pub(crate) const fn addrwidth(&self) -> u8 {
        match self.addrwidth {
            AddressWidth::FiveBytes  => 0b11,
            AddressWidth::FourBytes  => 0b10,
            AddressWidth::ThreeBytes => 0b01,
        }
    }

    /// Creates the secondary address.
    pub(crate) const fn secondary(&self) -> [u8; 5] {
        [
            self.subaddress[0],
            self.subaddress[1],
            self.subaddress[2],
            self.subaddress[3],

            match self.pipes[1] {
                Some(pipe) => pipe.sub,
                _ => 0,
            }
        ]
    }

    /// Creates the pipe configuration.
    pub(crate) const fn pipeconfig(&self) -> (u8, u8, u8, [u8; 6], [u8; 4]) {
        // Register words.
        let mut enaa = 0;
        let mut enrx = 0;
        let mut dynp = 0;

        // Width registers.
        let mut width = [0; 6];

        // Address registers.
        let mut addr = [0; 4];

        // Loop indexing.
        let mut i = 0;
        let l = self.pipes.len();

        while i < l {
            // Get the maybe pipe.
            let maybe = &self.pipes[i];

            if let Some(pipe) = maybe {
                // Enable the pipe.
                enrx |= 1 << i;

                // Set auto-acknowledge.
                if pipe.autoack { enaa |= 1 << i }

                // Set pipe width.
                match pipe.width {
                    Some(w) => width[i] = w,
                    _ => dynp |= 1 << i,
                }

                // Set address.
                if i > 1 { addr[i - 2] = pipe.sub }
            }

            // Increase the index.
            i += 1;
        }

        (enaa, enrx, dynp, width, addr)
    }

    /// Creates the features register.
    pub(crate) const fn features(&self) -> u8 {
        0b111
    }
}
