extern crate kad4;
pub mod filesystem;
pub mod daemon;
pub mod dht;
pub mod socket;

use std::collections::{BTreeMap, HashMap};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use filesystem::kfs::KFS;
use fuser::{FileType, MountOption};
use kad4::kad::kademlia_base::KademliaBase;
use crate::filesystem::inter::node::{Data, Node};
use crate::dht::kademlia::Kademlia;
use crate::socket::utp_socket::{UtpSocket};

//echo -n "hello" >/dev/udp/localhost/8080

fn main() {

    //self.running.store(true, Ordering::Relaxed);


    thread::spawn(|| {
        let mut utp_socket = UtpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind.");
        let (packet, src) = utp_socket.receive();

        println!("{}", src.to_string());
    });

    sleep(Duration::from_secs(2));

    let mut utp_socket = UtpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7072))).expect("Failed to bind.");
    utp_socket.send_to(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070)), "asdasd".as_bytes());

    loop {

    }

    /*
    let kad = Kademlia::default();//::try_from("Kademlia").unwrap();
    kad.get_routing_table().lock().unwrap().set_secure_only(false);
    kad.get_server().lock().unwrap().set_allow_bogon(true);

    kad.bind(6435);


    loop {
        sleep(Duration::from_secs(10));
        let routing_table = kad.get_routing_table().lock().unwrap();
        println!("CONSENSUS: {}  {}  {}",
                 routing_table.get_derived_uid().to_string(),
                 routing_table.get_consensus_external_address().to_string(),
                 routing_table.all_nodes().len());
    }
    */


    /*
    message types
    JOIN_NETWORK

    socket2
    LIST_FILES = BASED OFF OF FOLDER HASH
    FILE_CHANGE
    READ
    WRITE

    */







    /*
    let mut files: Vec<Box<dyn File>> = Vec::new();
    files.push(Box::new(KFile::new("hello_world.txt", 100)));

    let mut dir = KDirectory::new("test");
    dir.add_file(Box::new(KFile::new("new.txt", 100)));
    files.push(Box::new(dir));
    */

    /*
    let mut files = HashMap::new();

    let mut children = BTreeMap::new();
    children.insert("hello_world.txt".to_string(), 2 as u64);
    children.insert("new.txt".to_string(), 3 as u64);
    children.insert("test".to_string(), 4 as u64);

    files.insert(1, Node {
        data: Data {
            //name: ".".to_string(),
            //content: None,
            kind: FileType::Directory,
            size: 0
        },
        children: Some(children),
        parent: 0
    });

    files.insert(2, Node {
        data: Data {
            //name: "hello_world.txt".to_string(),
            kind: FileType::RegularFile,
            size: 11
        },
        children: None,
        parent: 1
    });

    files.insert(3, Node {
        data: Data {
            //name: "new.txt".to_string(),
            kind: FileType::RegularFile,
            size: 3
        },
        children: None,
        parent: 1
    });


    let mut children = BTreeMap::new();
    children.insert("asd".to_string(), 5 as u64);

    files.insert(4, Node {
        data: Data {
            //name: "test".to_string(),
            kind: FileType::Directory,
            size: 0
        },
        children: Some(children),
        parent: 1
    });


    files.insert(5, Node {
        data: Data {
            //name: "asd".to_string(),
            //content: None,
            kind: FileType::Directory,
            size: 0
        },
        children: Some(BTreeMap::new()),
        parent: 4
    });
    */

















    /*
    let mountpoint = "/media/test";
    let mut options = [
        MountOption::RW,
        MountOption::FSName("KFS".to_string()),
        MountOption::Async
        //MountOption::AutoUnmount
    ];
    fuser::mount2(KFS::default(), mountpoint, &options).unwrap();
    */
}
