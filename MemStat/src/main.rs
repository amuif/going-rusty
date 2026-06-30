use num_format::{Locale, ToFormattedString};
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    get_ram_size(&sys);
    get_running_apps(&mut sys);
}

fn get_ram_size(sys: &System) {
    let availible_ram: u64 = sys.total_memory() / 1024 / 1024;
    let used_ram: u64 = sys.used_memory() / 1024 / 1024;

    println!(
        "Total RAM: {} MB",
        availible_ram.to_formatted_string(&Locale::en)
    );
    println!("Used RAM: {} MB", used_ram.to_formatted_string(&Locale::en));
}

fn get_running_apps(sys: &mut System) {
    // time to calculate
    std::thread::sleep(std::time::Duration::from_millis(500));
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let mut processes: Vec<_> = sys.processes().values().collect();

    // Sort the processes by memory usage in descending order
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

    println!("{:<10} {:<30} {:<10}", "PID", "Name", "Memory (MB)");
    println!("{:-<55}", "-");
    for process in processes.iter().take(15) {
        let mem_mb: u64 = process.memory() / 1024 / 1024;
        println!(
            "{:<10} {:<30} {:<10}",
            process.pid(),
            process.name().to_string_lossy(),
            mem_mb
        );
    }
}
