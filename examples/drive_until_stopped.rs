use anyhow::Result;
use kobuki_interface::{
    serial_port::SerialPortHandler,
    tx::{ByteStream, commands},
};
use std::time::Duration;
use tokio::time::{Instant, sleep};
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("Driving until the bumper is activated...");

    let port = tokio_serial::new("/dev/kobuki", 115200)
        .timeout(Duration::from_millis(1024))
        .open_native_async()?;
    let serial = SerialPortHandler::new(port);

    let mut rx = serial.subscribe();
    let mut stop = false;
    let mut last_base_ctrl = Instant::now();
    loop {
        tokio::select! {
            feedback = rx.recv() => {
                if let Some(bsd) = feedback?.basic_sensor_data {
                    if !bsd.bumper.is_empty() {
                        println!("Bumper activated: {}", bsd.bumper);
                        stop = true;
                    }
                    if !bsd.wheel_drop.is_empty() {
                        println!("Wheel drop detected: {}", bsd.wheel_drop);
                        stop = true;
                    }
                }
                if stop {
                    break;
                }
            }
            _ = tokio::time::sleep_until(last_base_ctrl + Duration::from_secs(1)) => {
                last_base_ctrl = Instant::now();
                serial
                    .send_command(ByteStream::builder().subpayload(commands::BaseControl::new(100, 0)))
                    .await?;
            }

        }
    }

    serial
        .send_command(ByteStream::builder().subpayload(commands::BaseControl::new(0, 0)))
        .await?;

    serial
        .send_command(
            ByteStream::builder()
                .subpayload(commands::Sound::new(500.0, Duration::from_millis(500))),
        )
        .await?;

    sleep(Duration::from_secs(1)).await;
    Ok(())
}
