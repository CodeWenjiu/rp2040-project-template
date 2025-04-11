#![no_std]
#![no_main]

use defmt::info;

pub struct Executer {
    // Add fields here
}

impl Executer {
    pub fn new() {
        info!("Executer::new");
    }
}
