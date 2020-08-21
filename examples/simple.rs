extern crate transmission_rpc;

use dotenv::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result, RpcResponse, SessionGet};
use transmission_rpc::types::{Id, Nothing, TorrentAction};
use transmission_rpc::types::{TorrentAddArgs, TorrentAdded};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // setup session
    let url = env::var("TURL")?;
    let basic_auth = BasicAuth {
        user: env::var("TUSER")?,
        password: env::var("TPWD")?,
    };
    let client = TransClient::with_auth(&url, basic_auth);
    let response: Result<RpcResponse<SessionGet>> = client.session_get().await;
    match response {
        Ok(_) => println!("Yay!"),
        Err(_) => panic!("Oh no!"),
    }
    println!("Rpc reqsponse is ok: {}", response?.is_ok());

    // remove torrent
    let res: RpcResponse<Nothing> = client.torrent_remove(vec![Id::Id(1)], false).await?;
    println!("Remove result: {:?}", &res.is_ok());

    // add torrent
    let add: TorrentAddArgs = TorrentAddArgs {
        filename: Some(
            "https://releases.ubuntu.com/20.04/ubuntu-20.04.1-desktop-amd64.iso.torrent"
                .to_string(),
        ),
        ..TorrentAddArgs::default()
    };
    let res: RpcResponse<TorrentAdded> = client.torrent_add(add).await?;
    println!("Add result: {:?}", &res.is_ok());
    println!("response: {:?}", &res);

    // start torrent
    let res1: RpcResponse<Nothing> = client
        .torrent_action(TorrentAction::Start, vec![Id::Id(1)])
        .await?;
    println!("Start result: {:?}", &res1.is_ok());

    //
    Ok(())
}
