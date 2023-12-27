use h_analyzer_data::grpc_data_transfer;

use anyhow::Result;

pub use grpc_data_transfer::SeriesType;

use grpc_data_transfer::data_transfer2_d_client::DataTransfer2DClient;

pub struct HAnalyzerClient {
    data_trf_client:
        std::sync::Arc<tokio::sync::Mutex<DataTransfer2DClient<tonic::transport::Channel>>>,
}

impl HAnalyzerClient {
    pub async fn new() -> Self {
        let client = tokio::spawn(async move {
            let endpoints = ["http://192.168.64.2:50051"]
                .iter()
                .map(|a| tonic::transport::Channel::from_static(a));

            let channel = tonic::transport::Channel::balance_list(endpoints);
            DataTransfer2DClient::new(channel)
        })
        .await
        .unwrap();

        let arc = std::sync::Arc::new(tokio::sync::Mutex::new(client));

        Self {
            data_trf_client: arc,
        }
    }

    pub async fn connect_to_series(
        &mut self,
        //self: std::sync::Arc<Self>,
        name: &String,
        tp: SeriesType,
    ) -> Result<()> {
        let req = grpc_data_transfer::SeriesMetadata {
            id: Some(grpc_data_transfer::SeriesId { id: name.clone() }),
            element_type: tp as i32,
        };
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.data_trf_client);
            async move {
                let mut handle = handle.lock().await;
                let ret = handle.connect_to_new_series(req);
                ret.await.unwrap()
            }
        })
        .await
        .unwrap();
        Ok(())
    }

    pub async fn clear_series(&mut self, name: &String) -> Result<()> {
        let req = grpc_data_transfer::SeriesId { id: name.clone() };
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.data_trf_client);
            async move {
                let _ = handle.lock().await.clear_series(req).await;
            }
        })
        .await
        .unwrap();
        Ok(())
    }

    pub async fn send_point(&mut self, name: &String, x: f64, y: f64) -> Result<()> {
        let req = grpc_data_transfer::SendPoint2DRequest {
            id: Some(grpc_data_transfer::SeriesId { id: name.clone() }),
            point: Some(grpc_data_transfer::Point2D { x: x, y: y }),
        };
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.data_trf_client);
            async move {
                let _ = handle.lock().await.send_point(req).await;
            }
        })
        .await
        .unwrap();
        Ok(())
    }

    pub async fn send_pose_2d(&mut self, name: &String, x: f64, y: f64, theta: f64) -> Result<()> {
        let req = grpc_data_transfer::SendPose2DRequest {
            id: Some(grpc_data_transfer::SeriesId { id: name.clone() }),
            pose: Some(grpc_data_transfer::Pose2D {
                position: Some(grpc_data_transfer::Point2D { x: x, y: y }),
                theta: theta,
            }),
        };
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.data_trf_client);
            async move {
                let _ = handle.lock().await.send_pose2_d(req).await;
            }
        })
        .await
        .unwrap();
        Ok(())
    }

    pub async fn send_point_cloud_2d(
        &mut self,
        name: &String,
        xs: &Vec<f64>,
        ys: &Vec<f64>,
    ) -> Result<()> {
        let req = grpc_data_transfer::SendPointCloud2DRequest {
            id: Some(grpc_data_transfer::SeriesId { id: name.clone() }),
            pointcloud: Some(grpc_data_transfer::PointCloud2D {
                points: xs
                    .iter()
                    .zip(ys.iter())
                    .map(|(&x, &y)| grpc_data_transfer::Point2D { x: x, y: y })
                    .collect(),
            }),
        };
        tokio::spawn({
            let handle = std::sync::Arc::clone(&self.data_trf_client);
            async move {
                let _ = handle.lock().await.send_point_cloud2_d(req).await;
            }
        })
        .await
        .unwrap();
        Ok(())
    }
}
