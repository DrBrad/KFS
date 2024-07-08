/*
    SOCKETS
    - ASYNC
    - CAN HANDLE MULTIPLE CONCURRENT READS AND WRITES

    handshake
    - both parties must send handshake
    +----------------------------------+--------------------------+------------------------+----------------------+
    | Protocol Header (20 bytes)       | Reserved Bytes (8 bytes) | Info Hash (20 bytes)   | Peer ID (20 bytes)   |
    +----------------------------------+--------------------------+------------------------+----------------------+
    | "BitTorrent protocol" (19 bytes) | 0x00 0x00 ... 0x00       | [Info Hash] (20 bytes) | [Peer ID] (20 bytes) |
    +----------------------------------+--------------------------+------------------------+----------------------+

    request_list

    response_list

    request_file

    response_file

    request_write

    response_write
*/

pub trait Socket {

    fn connect(&self);

    fn handshake(&self);

    fn get_list(&self);

    fn write(&self);

    fn read(&self);
}
