use pcap;

fn main() {

   let mut cap = pcap::Device::lookup().unwrap().open().unwrap();

   while let Ok(packet) = cap.next() {
       println!("received packet! {:?}", packet)
   }

    let list = pcap::Device::list().unwrap();

    dbg!(list);

}