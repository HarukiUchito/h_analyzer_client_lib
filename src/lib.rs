use h_analyzer_grpc::grpc_data_transfer;

use anyhow::Result;

use grpc_data_transfer::data_transfer2_d_client::DataTransfer2DClient;

pub struct HAnalyzerClient {
    pub data_trf_client: DataTransfer2DClient<tonic::transport::Channel>,
    pub runtime: tokio::runtime::Runtime,
}

impl HAnalyzerClient {
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let client = rt.block_on(async move {
            let endpoints = ["http://192.168.64.2:50051"]
                .iter()
                .map(|a| tonic::transport::Channel::from_static(a));

            let channel = tonic::transport::Channel::balance_list(endpoints);
            DataTransfer2DClient::new(channel)
        });

        Self {
            data_trf_client: client,
            runtime: rt,
        }
    }

    pub fn connect_to_series(&mut self) -> Result<()> {
        let req = grpc_data_transfer::SeriesId {
            id: "test".to_string(),
        };
        self.runtime
            .block_on(self.data_trf_client.connect_to_new_series(req))?;
        Ok(())
    }

    pub fn clear_series(&mut self) -> Result<()> {
        let req = grpc_data_transfer::SeriesId {
            id: "test".to_string(),
        };
        self.runtime
            .block_on(self.data_trf_client.clear_series(req))?;
        Ok(())
    }

    pub fn send_point(&mut self, x: f64, y: f64) -> Result<()> {
        let req = grpc_data_transfer::SendPoint2DRequest {
            id: Some(grpc_data_transfer::SeriesId {
                id: "test".to_string(),
            }),
            point: Some(grpc_data_transfer::Point2D { x: x, y: y }),
        };
        println!("request sent");
        self.runtime
            .block_on(self.data_trf_client.send_point(req))?;
        Ok(())
    }
}
