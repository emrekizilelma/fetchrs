use sysinfo::System;

fn main() {

    let total_mem = System::new_all();

    println!("System name:       {:?}", System::name());
    println!("System OS version: {:?}", System::os_version());
    println!("CPU Arch:          {:?}", System::cpu_arch());
    println!("Kernel version:    {:?}", System::kernel_version());
    println!("Total Memory:      {}", total_mem.total_memory());
}