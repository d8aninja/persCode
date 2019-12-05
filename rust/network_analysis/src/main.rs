use pcap::{Device, Capture};
use futures::executor::block_on;

async fn capture_1() {
    let interface_1 = Device::lookup().unwrap();

    let mut cap1 = Capture::from_device(interface_1)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open().unwrap();

    while let Ok(packet) = cap1.next() {
        println!("Received packet on Capture 1! {:?}", packet);
    }
}

async fn capture_2() {
    let interface_2 = Device::lookup().unwrap();

    let mut cap2 = Capture::from_device(interface_2)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open().unwrap();

    while let Ok(packet) = cap2.next() {
        println!("Received packet on Capture 2! {:?}", packet);
    }
}

async fn async_main() {
    let f1 = capture_1();
    let f2 = capture_2();

    futures::join!(f1, f2)
}

fn main() {
    block_on(async_main())
}