use h_analyzer_grpc::grpc_data_transfer;

use anyhow::Result;

pub use grpc_data_transfer::SeriesType;

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

    pub fn connect_to_series(&mut self, name: &String, tp: SeriesType) -> Result<()> {
        let req = grpc_data_transfer::SeriesMetadata {
            id: Some(grpc_data_transfer::SeriesId { id: name.clone() }),
            element_type: tp as i32,
        };
        self.runtime
            .block_on(self.data_trf_client.connect_to_new_series(req))?;
        Ok(())
    }

    pub fn clear_series(&mut self, name: &String) -> Result<()> {
        let req = grpc_data_transfer::SeriesId { id: name.clone() };
        self.runtime
            .block_on(self.data_trf_client.clear_series(req))?;
        Ok(())
    }

    pub fn send_point(&mut self, name: &String, x: f64, y: f64) -> Result<()> {
        let req = grpc_data_transfer::SendPoint2DRequest {
            id: Some(grpc_data_transfer::SeriesId { id: name.clone() }),
            point: Some(grpc_data_transfer::Point2D { x: x, y: y }),
        };
        self.runtime
            .block_on(self.data_trf_client.send_point(req))?;
        Ok(())
    }

    pub fn send_pose_2d(&mut self, name: &String, x: f64, y: f64, theta: f64) -> Result<()> {
        let req = grpc_data_transfer::SendPose2DRequest {
            id: Some(grpc_data_transfer::SeriesId { id: name.clone() }),
            pose: Some(grpc_data_transfer::Pose2D {
                position: Some(grpc_data_transfer::Point2D { x: x, y: y }),
                theta: theta,
            }),
        };
        self.runtime
            .block_on(self.data_trf_client.send_pose2_d(req))?;
        Ok(())
    }
}
