use sysinfo::System;
use num_format::{Locale, ToFormattedString};

fn main() {
    let mut sys = System::new_all();
    let availible_ram: u64 = sys.total_memory() / 1024 / 1024;
    let used_ram: u64 = sys.used_memory() / 1024 / 1024;

    sys.refresh_all();
    println!("Total RAM: {} MB", availible_ram.to_formatted_string(&Locale::en));
    println!("Used RAM: {} MB", used_ram.to_formatted_string(&Locale::en));
}
