use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use anyhow::Result;
use tokio::time::timeout;
use tonic::transport::{Channel, Endpoint};
use tonic_grpc_load_balance::pb::msg_service_client::MsgServiceClient;
use tower::discover::Change;
use tracing::{info, Level};
use tonic_grpc_load_balance::config::Config;
use tonic_grpc_load_balance::pb::{Msg, SendMsgRequest};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_max_level(Level::INFO)
        .init();

    let config = Config::new("tonic_grpc_load_balance/config.yml")?;
    let endpoints = config.endpoints.clone();

    let (channel, rx) = Channel::balance_channel(10);
    let mut client = MsgServiceClient::new(channel);

    let done = Arc::new(AtomicBool::new(false));
    let demo_done = done.clone();


    // 模拟节点动态添加/删除
    tokio::spawn(async move{

        let mut i =1 ;
        for endpoint in endpoints.iter() {
            // http://127.0.0.1:50051
            let endpoint = format!("http://{}", endpoint);
            let endpoint = Endpoint::from_str(&endpoint).expect("valid endpoint");
            let change = Change::Insert(i, endpoint);
            let res = rx.send(change).await;
            info!("添加第{}个节点, result:  {:?}",i,res);
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            i+=1;
        }


        // 删除节点
        for _ in endpoints {
            let change = Change::Remove(i);
            let res = rx.send(change).await;
            info!("删除第{}个节点, result: {:?}",i,res);
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            i+=1;
        }


        demo_done.swap(true,SeqCst);

    });

    while !done.load(SeqCst) {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let mut message = Msg::default();
        message.send_id = "a123".to_string();
        message.recv_id = "x-123456".to_string();
        let req = SendMsgRequest{
            message: Some(message),
        };


        let rx = client.send_msg_to_user(req);
        if let Ok(resp) = timeout(tokio::time::Duration::from_secs(10), rx).await {
            info!("RESPONSE={:?}", resp);
        } else {
            info!("did not receive value within 10 secs");
        }

    }

    info!("client done");


    Ok(())
}
