use core_affinity::{self, CoreId};
use crossbeam::thread;
use dotenv::dotenv;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{FuturesUnordered, StreamExt},
};
use serde_json::{from_reader, Value};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::time::{Duration, Instant};
use transmission_rpc::types::{BasicAuth, Result, RpcResponse, SessionGet};
use transmission_rpc::types::{Id, Nothing, TorrentAction};
use transmission_rpc::types::{TorrentAddArgs, TorrentAdded};
use transmission_rpc::TransClient;

/// Read setup for NF only.
pub fn read_setup(file_path: String) -> Option<String> {
    let file = fs::File::open(file_path.clone()).expect("file should open read only");
    let read_json = file_path + "should be proper JSON";
    let json: Value = from_reader(file).expect(&read_json);

    let setup: Option<String> =
        match serde_json::from_value(json.get("setup").expect("file should have setup").clone()) {
            Ok(val) => Some(val),
            Err(e) => {
                println!("Malformed JSON response: {}", e);
                None
            }
        };

    if setup.is_some() {
        return setup;
    } else {
        println!("Setup: {:?} is None", setup);
        return None;
    }
}

/// Get the parameters for running p2p experiments.
///
/// 1 torrent job in total -- 3% pktgen sending rate
/// 5 torrent job in total -- 13% pktgen sending rate
/// 10 torrent job in total -- 25% pktgen sending rate
/// 20 torrent job in total -- 50% pktgen sending rate
/// 30 torrent job in total -- 75% pktgen sending rate
/// 40 torrent job in total -- 100% pktgen sending rate
pub fn p2p_retrieve_param(setup_val: usize) -> Option<usize> {
    let mut map = HashMap::new();
    map.insert(2, 1);
    map.insert(1, 10);
    map.insert(3, 50);
    map.insert(4, 100);
    map.insert(5, 150);
    map.insert(6, 200);

    map.insert(11, 1);
    map.insert(12, 1);
    map.insert(13, 1);
    map.insert(14, 1);
    map.insert(15, 1);
    map.insert(16, 1);
    map.insert(17, 1);
    map.insert(18, 1);
    map.insert(19, 1);
    map.insert(20, 1);

    map.remove(&setup_val)
}

pub fn p2p_fetch_workload(setup_val: usize) -> Option<&'static str> {
    let mut map = HashMap::new();
    map.insert(1, "/home/jethros/dev/pvn/utils/workloads/p2p-workload.json");
    map.insert(2, "/home/jethros/dev/pvn/utils/workloads/p2p-workload.json");
    map.insert(3, "/home/jethros/dev/pvn/utils/workloads/p2p-workload.json");
    map.insert(4, "/home/jethros/dev/pvn/utils/workloads/p2p-workload.json");
    map.insert(5, "/home/jethros/dev/pvn/utils/workloads/p2p-workload.json");
    map.insert(6, "/home/jethros/dev/pvn/utils/workloads/p2p-workload.json");

    map.insert(
        11,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-1.json",
    );
    map.insert(
        12,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-2.json",
    );
    map.insert(
        13,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-3.json",
    );
    map.insert(
        14,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-4.json",
    );
    map.insert(
        15,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-5.json",
    );
    map.insert(
        16,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-6.json",
    );
    map.insert(
        17,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-7.json",
    );
    map.insert(
        18,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-8.json",
    );
    map.insert(
        19,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-9.json",
    );
    map.insert(
        20,
        "/home/jethros/dev/pvn/utils/workloads/p2p-single-workload-10.json",
    );

    map.remove(&setup_val)
}

pub fn load_json(file_path: String) -> Vec<String> {
    let file = fs::File::open(file_path).expect("file should open read only");
    let json: Value = from_reader(file).expect("file should be proper JSON");

    let torrent_files = json
        .get("torrents_files")
        .expect("file should have time key")
        .clone();
    // println!("\ntorrent_files {:?}", torrent_files);

    let torrents: Vec<String> = serde_json::from_value(torrent_files).unwrap();
    // println!("\ntorrents {:?}", torrents);
    torrents
}

pub fn create_transmission_client() -> Result<TransClient> {
    dotenv().ok();
    // env_logger::init();

    // setup session
    let url: String = env::var("TURL")?;
    let basic_auth = BasicAuth {
        user: env::var("TUSER")?,
        password: env::var("TPWD")?,
    };
    let client = TransClient::with_auth(&url, basic_auth);
    Ok(client)
}

pub async fn run_all_torrents(
    p2p_param: usize,
    client: TransClient,
    mut workload: Vec<String>,
) -> Result<()> {
    let mut futures: FuturesUnordered<BoxFuture<Result<RpcResponse<TorrentAdded>>>> =
        FuturesUnordered::new();

    for (pos, t) in workload.iter().enumerate() {
        println!("Torrent at position {}: {:?}", pos, t);
        if pos >= 10 {
            println!("exiting with {}", pos);
            break;
        }
        // add torrent
        let add: TorrentAddArgs = TorrentAddArgs {
            filename: Some(t.to_string()),
            ..TorrentAddArgs::default()
        };

        futures.push(Box::pin(client.torrent_add(add)));
    }

    while let Some(result) = futures.next().await {
        match result {
            Ok(val) => println!("ok {:?}", val),
            Err(e) => eprintln!("err {}", e),
        }
    }

    Ok(())
}
