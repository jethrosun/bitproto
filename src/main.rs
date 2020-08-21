extern crate core_affinity;
extern crate crossbeam;
extern crate futures;
extern crate serde_json;

use crate::utils::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

mod utils;

fn main() {
    // setup for this run
    let setup_val = read_setup("/home/jethros/setup".to_string()).unwrap();
    let p2p_param = p2p_retrieve_param(setup_val.parse::<usize>().unwrap()).unwrap();

    // Measurement code
    //
    // NOTE: Store timestamps and calculate the delta to get the processing time for individual
    // packet is disabled here (TOTAL_MEASURED_PKT removed)

    // Workload and States for P2P NF
    //
    // 1, 10, 20, 40, 50, 75, 100, 150, 200
    let workload = p2p_fetch_workload(setup_val.parse::<usize>().unwrap()).unwrap();
    println!("{:?}", workload);
    let mut workload = load_json(workload.to_string());

    // Fixed transmission setup
    let torrents_dir = "/home/jethros/dev/pvn/utils/workloads/torrent_files/";

    let mut pivot = 0 as usize;
    let now = Instant::now();
    let mut start = Instant::now();

    let mut workload_exec = true;
    // let mut torrent_list = Vec::new();
    if workload_exec {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(add_all_torrents(
            p2p_param,
            workload.clone(),
            torrents_dir.to_string(),
        ));
        rt.block_on(run_all_torrents());
        workload_exec = false;
    }
}
