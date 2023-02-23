//! Configuration of a single data pipe for the NRF24L01 device.



#[derive(Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub struct PipeConfig {
    /// Pipe RX subaddress.
    pub sub: u8,

    /// Pipe Auto-Acknowledge.
    pub autoack: bool,

    /// Pipe enabled status.
    pub enabled: bool,

    /// Payload length of this pipe.
    /// Dynamic length if `None`.
    pub packetlen: Option<u8>,
}

impl PipeConfig {
    /// Creates a new empty PipeConfig.
    #[inline(always)]
    pub const fn empty() -> Self {
        PipeConfig { sub: 0u8, autoack: false, enabled: false, packetlen: None, }
    }

    /// Creates a new configured PipeConfig.
    pub const fn configured(sub: u8, autoack: bool, enabled: bool, packetlen: Option<u8>) -> Self {
        PipeConfig { sub, autoack, enabled, packetlen, }
    }

    /// Enables the PipeConfig.
    #[inline(always)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the PipeConfig.
    #[inline(always)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Sets the address of the PipeConfig.
    #[inline(always)]
    pub fn address(&mut self, address: u8) {
        self.sub = address;
    }

    /// Sets the auto-acknowledge of the PipeConfig.
    #[inline(always)]
    pub fn autoack(&mut self, auto: bool) {
        self.autoack = auto;
    }

    /// Sets the payload length of the pipe.
    #[inline(always)]
    pub fn packetlen(&mut self, len: u8) {
        self.packetlen = Some( len );
    }

    /// Sets the payload length to dynamic.
    #[inline(always)]
    pub fn dynlength(&mut self) {
        self.packetlen = None;
    }
}
