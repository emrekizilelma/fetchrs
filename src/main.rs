use sysinfo::System;

fn main() {
    println!("CPU Arch:          {:?}", System::cpu_arch());
    println!("System name:       {:?}", System::name());
    println!("System OS version: {:?}", System::os_version());
    println!("System host name:  {:?}", System::host_name());
}