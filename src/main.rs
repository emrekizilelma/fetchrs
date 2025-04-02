use sysinfo::System;

fn main() {

    let total_mem = System::new_all();
    
    println!("System name:       {}", System::name().unwrap());
    println!("System OS version: {}", System::os_version().unwrap());
    println!("CPU Arch:          {}", System::cpu_arch().unwrap());
    println!("Kernel version:    {}", System::kernel_version().unwrap());
    println!("Hostname           {:?}", System::host_name().unwrap());
    println!("Total Memory:      {}", total_mem.total_memory());
}
