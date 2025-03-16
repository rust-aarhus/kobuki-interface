use anyhow::Result;
use kobuki_interface::serial_port::SerialPortHandler;
use std::time::Duration;
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("Decoding feedback messages...");

    let port = tokio_serial::new("/dev/kobuki", 115200)
        .timeout(Duration::from_millis(1024))
        .open_native_async()?;
    let serial = SerialPortHandler::new(port);
    let mut rx = serial.subscribe();

    loop {
        let feedback = rx.recv().await?;
        if let Some(bsd) = feedback.basic_sensor_data {
            if !bsd.bumper.is_empty() {
                println!("Bumper activated: {}", bsd.bumper);
            }
            if !bsd.wheel_drop.is_empty() {
                println!("Wheel drop detected: {}", bsd.wheel_drop);
            }
            if !bsd.button.is_empty() {
                println!("Button pressed: {}", bsd.button);
            }
        }
    }
}
