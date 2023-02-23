//! NRF24L01(+) pipe receiver configuration.



#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Pipe {
    /// Pipe subaddress.
    pub sub: u8,

    /// Pipe auto-acknowledge.
    pub autoack: bool,

    /// Pipe byte width.
    /// Dynamic width if `None`.
    pub width: Option<u8>,
}

impl Pipe {
    /// Configures a dynamic length pipeline.
    pub const fn dynamic(sub: u8, autoack: bool) -> Pipe {
        Pipe { sub, autoack, width: None }
    }

    /// Configures a set length pipeline.
    pub const fn sized(sub: u8, autoack: bool, width: u8) -> Pipe {
        Pipe { sub, autoack, width: Some(width) }
    }
}
