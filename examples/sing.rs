use anyhow::Result;
use kobuki_interface::{
    serial_port::SerialPortHandler,
    tx::{ByteStream, commands},
};
use std::time::Duration;
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("Singing a song...");

    let song = [
        (261, 400),
        (330, 300),
        (349, 200),
        (392, 400),
        (349, 300),
        (329, 200),
        (220, 400),
        (246, 300),
        (261, 200),
        (220, 400),
        (246, 300),
        (293, 400),
        (349, 200),
        (392, 400),
        (440, 300),
        (493, 200),
        (523, 400),
    ];

    let port = tokio_serial::new("/dev/kobuki", 115200)
        .timeout(Duration::from_millis(1024))
        .open_native_async()?;
    let serial = SerialPortHandler::new(port);

    for (note, duration) in &song {
        let duration = Duration::from_millis(*duration / 2);
        let mut bs = ByteStream::builder();
        bs = bs.subpayload(commands::Sound::new(*note as f32, duration));
        serial.send_command(bs).await?;
        tokio::time::sleep(duration).await;
    }

    Ok(())
}
