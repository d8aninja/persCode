use std::collections::HashMap;
use std::time::Duration;
use systemstat::{System, Platform};

fn main() {
    let sys = System::new();

    let mem_keys = vec!["MemUsed", "MemTotal"];
    let mem_stats: HashMap<&str, bytesize::ByteSize> = if let Ok(mem) = sys.memory() {
        let mem_values = vec![mem.total - mem.free, mem.total]; 
        mem_keys.into_iter().zip(mem_values.into_iter()).collect()
    } else {
        HashMap::new()
    };
    
    let cpu_keys = vec!["user", "nice", "system", "interrupt", "idle"];
    let cpu_stats: HashMap<&str, f32> = if let Ok(cpu) = sys.cpu_load_aggregate() {
        let cpu = cpu.done().unwrap();
        let cpu_values = vec![cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0]; 
        cpu_keys.into_iter().zip(cpu_values.into_iter()).collect()
    } else {
        HashMap::new()
    };

    println!("{:?}", mem_stats);
    println!("{:?}", cpu_stats);
}