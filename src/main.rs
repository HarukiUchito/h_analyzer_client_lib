use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cl = h_analyzer_client_lib::HAnalyzerClient::new("http://localhost:50051").await;

    let mut wf = h_analyzer_data::WorldFrame::new(0, 1.52);
    let mut ego = h_analyzer_data::Entity::new();
    let m = h_analyzer_data::Measurement::Pose2D(h_analyzer_data::Pose2D::new(0.0, 1.0, 2.0));
    ego.add_measurement("pose".to_string(), m);
    wf.add_entity("Ego".to_string(), ego);

    cl.register_new_world(&"test".to_string()).await.unwrap();
    cl.send_world_frame(wf).await.unwrap();

    Ok(())
}
