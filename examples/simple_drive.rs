use anyhow::Result;
use kobuki_interface::{
    serial_port::SerialPortHandler,
    tx::{ByteStream, commands},
};
use std::time::Duration;
use tokio::time::sleep;
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("Driving a bit...");

    let port = tokio_serial::new("/dev/kobuki", 115200)
        .timeout(Duration::from_millis(1024))
        .open_native_async()?;
    let serial = SerialPortHandler::new(port);

    // Notice that the base control needs to be set regularly to keep the robot moving
    for _ in 0..10 {
        serial
            .send_command(ByteStream::builder().subpayload(commands::BaseControl::new(100, 100)))
            .await?;
        sleep(Duration::from_secs(1)).await;
    }
    serial
        .send_command(ByteStream::builder().subpayload(commands::BaseControl::new(0, 0)))
        .await?;

    // allow the last command to be processed before terminating
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
