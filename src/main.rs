use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let name = "test2".to_string();
    let mut cl = h_analyzer_client_lib::HAnalyzerClient::new().await;
    cl.connect_to_series(&name, h_analyzer_client_lib::SeriesType::Point2d)
        .await
        .unwrap();
    cl.clear_series(&name).await.unwrap();
    for i in 0..10 {
        cl.send_point(&name, 0.1 * i as f64, 0.2 * i as f64)
            .await
            .unwrap();
    }

    let pcl = "points".to_string();
    cl.connect_to_series(&pcl, h_analyzer_client_lib::SeriesType::PointCloud)
        .await
        .unwrap();
    cl.send_point_cloud_2d(&pcl, &vec![1.0, 2.0, 3.0], &vec![-3.0, 3.0, 4.2])
        .await
        .unwrap();

    let mut wf = h_analyzer_data::WorldFrame::new(1.52);
    let mut ego = h_analyzer_data::Entity::new();
    let m = h_analyzer_data::Measurement::Pose2D(h_analyzer_data::Pose2D::new(0.0, 1.0, 2.0));
    ego.add_measurement("pose".to_string(), m);
    wf.add_entity("Ego".to_string(), ego);

    cl.send_world_frame(wf).await.unwrap();

    Ok(())
}
