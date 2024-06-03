use h_analyzer_data::grpc_data_transfer;

use anyhow::Result;
use grpc_data_transfer::data_transfer2_d_client::DataTransfer2DClient;

use polars::prelude::*;

pub struct HAnalyzerClient {
    data_trf_client:
        std::sync::Arc<tokio::sync::Mutex<DataTransfer2DClient<tonic::transport::Channel>>>,
    polars_svc_client: std::sync::Arc<
        tokio::sync::Mutex<
            h_analyzer_data::grpc_fs::polars_service_client::PolarsServiceClient<
                tonic::transport::Channel,
            >,
        >,
    >,
}

impl HAnalyzerClient {
    pub async fn new(addr: &'static str) -> Self {
        let client = tokio::spawn(async move {
            let addr = [addr];
            let endpoints = addr
                .iter()
                .map(|a| tonic::transport::Channel::from_static(a));

            let channel = tonic::transport::Channel::balance_list(endpoints);
            DataTransfer2DClient::new(channel)
        })
        .await
        .unwrap();

        let arc = std::sync::Arc::new(tokio::sync::Mutex::new(client));

        let client1 = tokio::spawn(async move {
            let addr = [addr];
            let endpoints = addr
                .iter()
                .map(|a| tonic::transport::Channel::from_static(a));

            let channel = tonic::transport::Channel::balance_list(endpoints);
            h_analyzer_data::grpc_fs::polars_service_client::PolarsServiceClient::new(channel)
        })
        .await
        .unwrap();

        let arc1 = std::sync::Arc::new(tokio::sync::Mutex::new(client1));

        Self {
            data_trf_client: arc,
            polars_svc_client: arc1,
        }
    }

    pub async fn send_world_frame(
        &mut self,
        world_frame: h_analyzer_data::WorldFrame,
    ) -> Result<()> {
        let bytes = bincode::serialize(&world_frame).unwrap();
        let mut wf_data = Vec::new();
        const CHUNK_SIZE: i32 = 1024;
        let mut cnt = 0;
        let mut chunk = Vec::new();
        for byte in bytes {
            cnt += 1;
            chunk.push(byte);
            if cnt == CHUNK_SIZE {
                cnt = 0;
                wf_data.push(grpc_data_transfer::WorldFrameBytes {
                    data: chunk.clone(),
                });
                chunk.clear();
            }
        }
        if !chunk.is_empty() {
            wf_data.push(grpc_data_transfer::WorldFrameBytes {
                data: chunk.clone(),
            });
        }

        let request = tonic::Request::new(tokio_stream::iter(wf_data));
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.data_trf_client);
            async move {
                let mut handle = handle.lock().await;
                match handle.send_world_frame(request).await {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
        })
        .await
        .unwrap();
        Ok(())
    }

    pub async fn register_new_world(&mut self, name: &String) -> Result<()> {
        let req = grpc_data_transfer::WorldId { id: name.clone() };
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.data_trf_client);
            async move {
                let mut handle = handle.lock().await;
                let ret = handle.register_new_world(req);
                ret.await.unwrap()
            }
        })
        .await
        .unwrap();
        Ok(())
    }

    pub async fn register_data_frame(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn send_data_frame(&mut self, df: DataFrame) -> Result<()> {
        let bytes = bincode::serialize(&df).unwrap();
        let mut wf_data = Vec::new();
        const CHUNK_SIZE: i32 = 1024;
        let mut cnt = 0;
        let mut chunk = Vec::new();
        for byte in bytes {
            cnt += 1;
            chunk.push(byte);
            if cnt == CHUNK_SIZE {
                cnt = 0;
                wf_data.push(h_analyzer_data::grpc_fs::DataFrameBytes {
                    data: chunk.clone(),
                });
                chunk.clear();
            }
        }
        if !chunk.is_empty() {
            wf_data.push(h_analyzer_data::grpc_fs::DataFrameBytes {
                data: chunk.clone(),
            });
        }

        let request = tonic::Request::new(tokio_stream::iter(wf_data));
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.polars_svc_client);
            async move {
                let mut handle = handle.lock().await;
                match handle.send_data_frame(request).await {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
        })
        .await
        .unwrap();
        Ok(())
    }
}
