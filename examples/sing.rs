use anyhow::Result;
use kobuki_interface::tx::{ByteStream, commands};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
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

    let mut port = tokio_serial::new("/dev/kobuki", 115200)
        .timeout(Duration::from_millis(1024))
        .open()?;

    for (note, duration) in &song {
        let duration = Duration::from_millis(*duration / 2);
        let mut bs = ByteStream::builder();
        bs = bs.subpayload(commands::Sound::new(*note, duration));
        let d = bs.to_bytes();
        let _ = port.write(&d)?;
        tokio::time::sleep(duration).await;
    }

    Ok(())
}
