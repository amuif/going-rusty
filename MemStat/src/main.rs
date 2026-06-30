use num_format::{Locale, ToFormattedString};
use sysinfo::{Process, System};

struct MemoryInfo {
    total_ram: u64,
    used_ram: u64,
}
struct ProcessInfo<'a> {
    processes: Vec<&'a Process>,
}

fn main() {
    let mut sys = System::new_all();
    let MemoryInfo {
        total_ram,
        used_ram,
    } = get_ram_size(&sys);
    println!("Total ram:{}, Used ram:{}", total_ram, used_ram);
    println!("");
    let ProcessInfo { processes } = get_running_apps(&mut sys);

    println!("{:<10} {:<24} {:<10}", "PID", "Name", "Memory (MB)");
    println!("{:-<55}", "-");
    for process in processes.iter().take(15) {
        let mem_mb: u64 = process.memory() / 1024 / 1024;
        println!(
            "{:<10} {:<30} {:<10}",
            process.pid(),
            process.name().to_string_lossy(),
            mem_mb.to_formatted_string(&Locale::en)
        );
    }
}

fn get_ram_size(sys: &System) -> MemoryInfo {
    let total_ram: u64 = sys.total_memory() / 1024 / 1024;
    let used_ram: u64 = sys.used_memory() / 1024 / 1024;
    MemoryInfo {
        total_ram,
        used_ram,
    }
}

fn get_running_apps<'a>(sys: &'a mut System) -> ProcessInfo<'a> {
    std::thread::sleep(std::time::Duration::from_millis(500));
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));
    
    ProcessInfo { processes }
}