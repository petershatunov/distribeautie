use std::thread;
use std::time::Duration;

use crate::{config, networking, storage};

pub(crate) fn run() {
    if !config::is_masterhost() {
        return;
    };
    loop {
        println!("Scheduler is running...");
        thread::sleep(Duration::from_millis(config::get_sync_interval_ms()));
        sync_replicas();
    }
}

fn sync_replicas() {
    let all_items = storage::get_all_items();
    println!("items to replicate: {:?}", all_items);
    networking::update_replicas();
}
