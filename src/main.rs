use anyhow::Result;
use chrono::NaiveDate;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cl = h_analyzer_client_lib::HAnalyzerClient::new("http://localhost:50051").await;

    let df: DataFrame = df!(
        "integer" => &[1, 2, 3, 4, 5],
        "date" => &[1, 2, 3, 4, 5],
        "float" => &[4.0, 5.0, 600.0, 70.0, 8.0]
    )
    .unwrap();
    println!("{}", df);

    cl.register_data_frame("imu.csvs".to_string())
        .await
        .unwrap();
    cl.send_data_frame(df).await.unwrap();

    return Ok(());

    let mut wf = h_analyzer_data::WorldFrame::new(0, 1.52);
    let mut ego = h_analyzer_data::Entity::new();
    let m = h_analyzer_data::Measurement::Pose2D(h_analyzer_data::Pose2D::new(0.0, 1.0, 2.0));
    ego.add_measurement("pose".to_string(), m);
    wf.add_entity("Ego".to_string(), ego);

    cl.register_new_world(&"test".to_string()).await.unwrap();
    cl.send_world_frame(wf).await.unwrap();

    Ok(())
}
