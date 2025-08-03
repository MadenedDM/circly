use std::{
    collections::HashMap,
    error::Error,
    io::{ BufReader, BufWriter, Read, Write },
    net::{ Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream },
    sync::{ Arc, Mutex },
    thread::{ spawn, JoinHandle },
};

use hecs::{ World };
use uuid::Uuid;

const PORT: u16 = 9878;

// #[derive(Clone, Copy, Debug)]
// struct PlayerComponent {
//     owner: u32,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT);
    println!("Trying address, {address}");

    let listener: TcpListener = TcpListener::bind(address).expect("Binding failed.");
    println!("Server bound to: {address}");

    let world = Arc::new(Mutex::new(World::new()));
    // let players = Arc::new(Mutex::new(HashMap::<Uuid, Entity>::new()));

    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let mut connections = HashMap::<SocketAddr, Uuid>::new();

    {
        let mut data = world.lock().unwrap();
        data.spawn(());
    }

    loop {
        let (stream, addr) = listener.accept().unwrap();
        connections.insert(addr, Uuid::new_v4());

        for i in &connections {
            println!("{i:?}");
        }

        threads.push(
            spawn(move || {
                println!("New connection: {addr}");
                process(stream, addr);
            })
        );
    }
}

fn process(stream: TcpStream, addr: SocketAddr) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        let mut buf = [0u8; 16];
        reader.read(&mut buf).unwrap();

        if buf != [0u8; 16] {
            println!("{addr}: {:?}", buf);
        }

        let _ = writer.write(b"Hello!:)");

        //     match  {
        //         Ok(byt) => {println!("{byt}")},
        //         Err(e) =>
        //             match e.kind() {
        //                 ErrorKind::BrokenPipe => {
        //                     break;
        //                 }
        //                 ErrorKind::ConnectionAborted => {
        //                     break;
        //                 }
        //                 _ => (),
        //             }
        //     }
    }
}
