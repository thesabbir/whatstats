use clap::Parser;
use sysinfo::{System, SystemExt, DiskExt};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(name="WhatStats")]
#[clap(version = "1.0")]
#[clap(about = "Get system stats", long_about = None)]

struct Cli {
    #[clap(short, long)]
    visualize: bool
}
fn main() {
    

    let cli = Cli::parse();

    let mut sys = System::new_all();
    sys.refresh_all();

    for disk in sys.disks() {
        println!("{:?}, Available: {:?}", disk.mount_point(), disk.available_space());
    }

    println!("Total memory: {:?}mb", sys.total_memory() / 1024);
    println!("Free memory: {:?}mb", sys.free_memory() / 1024);
    println!("Uptime: {:?} Hours", sys.uptime()/60/60);
    
}
