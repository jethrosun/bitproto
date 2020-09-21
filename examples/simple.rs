extern crate shell_words;
use std::io::Result;

/// Run BitTorrent jobs via deluge console
pub fn bt_run_torrents(workload: &str, num_of_torrents: usize) -> Result<()> {
    for i in 1..(num_of_torrents + 1) {
        let mut argv = Vec::new();
        argv.push("deluge-console".to_string());
        argv.push("-c".to_string());
        argv.push("/home/jethros/bt_data/config".to_string());

        // let s =
        //     "\"add  $HOME/dev/pvn/utils/workloads/torrent_files/img".to_owned() + &i.to_string() + "_secret.torrent\"";
        let s = "add  $HOME/dev/pvn/utils/workloads/torrent_files/img".to_owned()
            + &i.to_string()
            + "_secret.torrent";
        argv.push(s);
        println!("Executing: {}", shell_words::join(&argv));

        // std::process::Command::new(&argv[0])
        //     .args(&argv[1..])
        //     .spawn()
        //     .expect("failed to start subprocess");
    }

    Ok(())
}

fn main() {
    let bt = bt_run_torrents("bogus", 3);
    //
}
