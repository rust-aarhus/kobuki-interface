use anyhow::Result;
use kobuki_interface::tx::{ByteStream, commands};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Driving a bit...");

    let mut port = tokio_serial::new("/dev/kobuki", 115200)
        .timeout(Duration::from_millis(1024))
        .open()?;

    let d = ByteStream::builder()
        .subpayload(commands::BaseControl::new(100, 100))
        .to_bytes();
    let _ = port.write(&d)?;

    sleep(Duration::from_secs(2)).await;

    let d = ByteStream::builder()
        .subpayload(commands::BaseControl::new(0, 0))
        .to_bytes();
    let _ = port.write(&d)?;

    Ok(())
}
