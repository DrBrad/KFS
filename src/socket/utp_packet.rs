const HEADER_SIZE: usize = 20; // Basic header size without extensions

// Define the uTP packet header
#[derive(Debug)]
pub struct UtpHeader {
    pub(crate) type_version: u8,
    pub(crate) extension: u8,
    pub(crate) connection_id: u16,
    pub(crate) timestamp: u32,
    pub(crate) timestamp_diff: u32,
    pub(crate) wnd_size: u32,
    pub(crate) seq_nr: u16,
    pub(crate) ack_nr: u16,
}

// Define the uTP packet structure
#[derive(Debug)]
pub struct UtpPacket {
    pub(crate) header: UtpHeader,
    pub(crate) payload: Vec<u8>,
}
