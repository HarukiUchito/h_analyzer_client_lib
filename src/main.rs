use anyhow::Result;

fn main() -> Result<()> {
    let mut cl = h_analyzer_client_lib::HAnalyzerClient::new();
    let name = "test2".to_string();
    cl.connect_to_series(&name)?;
    cl.clear_series(&name)?;
    for i in 0..10 {
        cl.send_point(&name, i as f64, 0.2 * i as f64)?;
    }

    cl.runtime.shutdown_background();
    Ok(())
}
