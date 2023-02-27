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

    info!("Start listening on {}", addr);

    let table: Arc<DashMap<String, Value>> = Arc::new(DashMap::new());

    loop {
        let (stream, addr) = listener.accept().await?;

        info!("Client connected {}", addr);
        let db = table.clone(); // 复制一份

        //    并发处理

        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

            while let Some(Ok(msg)) = stream.next().await {
                info!("Got a new command: {:?}", msg);

                let resp: CommandResponse = match msg.request_data {
                    Some(RequestData::Hset(cmd)) => hset(cmd, &db),
                    _ => unimplemented!(),
                };

                info!("Got response: {:?}", resp);
                stream.send(resp).await.unwrap();
            }
        });
    }
}

fn hset(cmd: Hset, db: &DashMap<String, Value>) -> CommandResponse {
    match cmd.pair {
        Some(Kvpair {
            key,
            value: Some(value),
        }) => {
            let old = db.insert(key, value).unwrap_or_default();
            old.into()
        }

        v => KvError::InvalidCommand(format!("hset: {:?}", v)).into(),
    }
}
