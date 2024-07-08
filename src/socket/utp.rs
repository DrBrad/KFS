use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::Arc;
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;

pub struct UTP {
    server: Option<Arc<UdpSocket>>
}

impl UTP {

    pub fn new() -> Self {
        Self {
            server: None
        }
    }

    pub fn start(&mut self, port: u16) {
        self.server = Some(Arc::new(UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))).expect("Failed to bind socket")));
        let (tx, rx) = channel();
        let sender = tx.clone();
        let server = Arc::clone(self.server.as_ref().unwrap());

        let receiver_handle = thread::spawn(move || {
            let mut buf = [0u8; 65535];

            while true {
                let (size, src_addr) = {
                    server.recv_from(&mut buf).expect("Failed to receive message")
                };

                sender.send((buf[..size].to_vec(), src_addr)).unwrap();

                /*
                let data = &buf[..size];

                let bytes = data.as_ptr();
                let len = data.len();
                forget(data);

                unsafe {
                    sender.send((from_raw_parts(bytes, len), src_addr)).expect("Failed to send packet to handler");
                }
                */
            }
        });


        let handler_handle = thread::spawn(move || {
            while true {
                match rx.try_recv() {
                    Ok((data, src_addr)) => {
                        //Self::on_receive(kademlia.as_mut(), data.as_slice(), src_addr);
                        println!("RECEIVED")
                    }
                    Err(TryRecvError::Empty) => {
                    }
                    Err(TryRecvError::Disconnected) => break
                }
            }
        });
    }
}
