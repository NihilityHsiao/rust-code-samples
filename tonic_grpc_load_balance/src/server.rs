use anyhow::{bail, Result};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tonic::{async_trait, Request, Response, Status};
use tonic::transport::Server;
use tracing::{debug, info, Level};
use tonic_grpc_load_balance::pb::{SendMsgRequest, SendMsgResponse};
use tonic_grpc_load_balance::*;
use tonic_grpc_load_balance::config::Config;

#[derive(Debug)]
pub struct MsgServer {
    id: usize,
    addr: SocketAddr,
}

#[async_trait]
impl pb::msg_service_server::MsgService for MsgServer {
    async fn send_message(
        &self,
        request: Request<SendMsgRequest>,
    ) -> std::result::Result<Response<SendMsgResponse>, Status> {
        let msg = request.into_inner().message;
        info!("server[{}] get request: {:?}",self.id, msg);

        let Some(msg) = msg else {
            return Ok(Response::new(SendMsgResponse {}))
        };
        info!("server[{}] send msg from {} to {}",self.id,msg.send_id, msg.recv_id);

        Ok(Response::new(SendMsgResponse {}))
    }

    async fn send_msg_to_user(
        &self,
        request: Request<SendMsgRequest>,
    ) -> std::result::Result<Response<SendMsgResponse>, Status> {
        let msg = request.into_inner().message;
        info!("server[{}] get request: {:?}",self.id, msg);

        let Some(msg) = msg else {
            return Ok(Response::new(SendMsgResponse {}))
        };
        info!("server[{}] send msg to user: {}",self.id, msg.recv_id);

        Ok(Response::new(SendMsgResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new("tonic_grpc_load_balance/config.yml")?;
    info!("config: {:?}",config);
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_max_level(Level::INFO)
        .init();
    let addrs = config.endpoints.clone();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let mut id = 1;
    for addr in addrs.iter() {
        let addr = addr.parse()?;
        let tx = tx.clone();

        info!("server[{}] start at: {}",id,addr);

        let server = MsgServer { id,addr };
        id+=1;
        let serve = Server::builder()
            .add_service(pb::msg_service_server::MsgServiceServer::new(server))
            .serve(addr);


        tokio::spawn(async move {
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }

            tx.send(()).unwrap();
        });
    }

    rx.recv().await;


    Ok(())
}
