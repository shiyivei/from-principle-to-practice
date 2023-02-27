use anyhow::Result;
use async_prost::AsyncProstStream;
use dashmap::DashMap;
use futures::prelude::*;

use kv_server::{
    command_request::RequestData, CommandRequest, CommandResponse, Hset, KvError, Kvpair, Value,
};

use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";

    let listener = TcpListener::bind(addr).await?;

    info!("Start Listening on {}", addr);

    loop {
        let (x, addr) = listener.accept().await?;

        info!("Client {:#?} connected", addr);

        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(x).for_async();

            while let Some(Ok(msg)) = stream.next().await {
                info!("Got a new command {:#?}", msg);

                let mut resp = CommandResponse::default();

                resp.status = 404;
                resp.message = "Not Found".to_string();
                stream.send(resp).await.unwrap()
            }

            info!("Client {:#?} disconnected", addr);
        });
    }
}
