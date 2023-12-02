use anyhow::Result;

fn main() -> Result<()> {
    let mut cl = h_analyzer_client_lib::HAnalyzerClient::new();
    cl.connect_to_series()?;
    for i in 0..10 {
        cl.send_point(i as f64, 0.2 * i as f64)?;
    }

    cl.runtime.shutdown_background();
    Ok(())
}
