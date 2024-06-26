use sysinfo::{Disks, Networks, System};
use std::thread;
use std::time::Duration;
use std::env; 

fn main() {
    println!("System profiler Version 0.0.1\nApostolos Chalis 2024\n");
   
    let mut recording = false; 
    let args: Vec<String> = env::args().collect();
    let flag_exists = args.len(); 

    if flag_exists == 2{
        let flag = &args[1]; // supports one CLI argument right now ( --record)
        if flag == "--record"{
            recording = true; 
        }
    }
      
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        // RAM usage
        println!("==> RAM.");
        println!("Used memory : {} bytes / {} bytes", sys.used_memory(), sys.total_memory());
        println!("RAM usage: {}%", calculate_used_ram(sys.used_memory(), sys.total_memory()));
        print!("\n");

        // CPU usage
        println!("==> CPU usage.");
        sys.refresh_cpu();
        let mut counter = 0;
        let mut cpu_usages: Vec<f64> = Vec::new();

        for cpu in sys.cpus() {
            counter += 1;
            print!("CPU {}: {}% ", counter, cpu.cpu_usage());
            cpu_usages.push(cpu.cpu_usage() as f64); 
        }
        print!("\n");

        let size_of_vector = cpu_usages.len() as u16; 
        println!("CPU utilization: {}", calculate_cpu_total_usage(cpu_usages, size_of_vector)); 
        print!("\n");

        // Network usage
        println!("\n==> Network usage.");
        let networks = Networks::new_with_refreshed_list();

        for (interface_name, data) in &networks {
            println!(
                "{interface_name}: {} B (down) / {} B (up)",
                data.total_received(),
                data.total_transmitted(),
            );
            println!("Packets/S (PPS): {}", calculate_pps(data.total_received(), data.total_transmitted()));
        }
        print!("\n");

        // Disks usage
        println!("==> Disks usage.");
        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            println!("{disk:?}");
        }

        // Control structure to funnel data on the record function
        if recording == true{
            record(); 
        }

        // Sleep for a while before refreshing
        thread::sleep(Duration::from_secs(1)); // Adjust refresh rate as needed
    }
}

fn calculate_used_ram(u_ram:u64, t_ram:u64) -> u16{
    // u_ram = Used RAM 
    // t_ram = Total RAM
    
    let div: f64 = u_ram as f64 / t_ram as f64;
    let percentage = (div * 100.0) as u16;

    percentage
}

fn calculate_cpu_total_usage(core_usages:Vec<f64>, cores:u16) -> f64{
    // u_cpu = Utilized CPU 
    // cores = Number of cores in the socket
    
    let f_cores = cores as f64;

    let total_core_usage:f64 = core_usages.iter().sum();
    let utilization = total_core_usage / f_cores;

    utilization
}

fn calculate_pps(up_net:u64, down_net:u64) -> u64{
    // up_net = Number of bytes that the network is sending
    // down_net = Number of bytes that the network is receiving
    
    let total_net = up_net + down_net; 
    let packets = total_net / 1518; // Packet size on ethernet connections TODO: Add packet
                                    // calculations for other interfaces
    packets
}

fn record(){
    println!("\nSession is being recorded.");
}

