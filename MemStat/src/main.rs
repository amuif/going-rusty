use sysinfo::{System};

fn main() {
   let mut sys = System::new_all();
   sys.refresh_all();
    println!("Total RAM: {} KB", sys.total_memory());
    println!("Used RAM: {} KB", sys.used_memory());

    // To check the RAM usage of your specific process
    let pid = sysinfo::get_current_pid().unwrap();
    if let Some(process) = sys.process(pid) {
        println!("Memory used by this process: {} KB", process.memory());
    }
}
