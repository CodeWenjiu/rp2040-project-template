#![no_std]
#![no_main]

struct executer {}

impl executer {
    pub fn new() -> Self {
        info!("Executer Created");
        executer {}
    }
}
