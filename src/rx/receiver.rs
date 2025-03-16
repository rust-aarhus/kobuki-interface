use super::{Feedback, FeedbackDecoder};
use log::error;
use tokio::{io::AsyncReadExt, sync::broadcast};
use tokio_serial::SerialStream;
use tokio_util::codec::Decoder;

pub struct Receiver {
    rx: broadcast::Receiver<Feedback>,
    task: tokio::task::JoinHandle<()>,
}

impl Receiver {
    pub fn new(port: SerialStream) -> Self {
        let (tx, rx) = broadcast::channel(10);
        let task = tokio::spawn(async move {
            if let Err(e) = Self::run(port, tx).await {
                error!("Error handling serial port: {:?}", e);
            }
        });
        Self { rx, task }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Feedback> {
        self.rx.resubscribe()
    }

    pub async fn run(
        mut port: SerialStream,
        tx: broadcast::Sender<Feedback>,
    ) -> std::io::Result<()> {
        let mut decoder = FeedbackDecoder {};
        let mut buf = bytes::BytesMut::new();

        loop {
            let size = port.read_buf(&mut buf).await?;
            if size == 0 {
                while let Some(frame) = decoder.decode_eof(&mut buf)? {
                    tx.send(frame).ok(); // send will give error if no subscribers - ignore errors
                }
                return Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    "Connection closed by peer",
                ));
            }

            while let Some(frame) = decoder.decode(&mut buf)? {
                tx.send(frame).ok(); // send will give error if no subscribers - ignore errors
            }
        }
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        self.task.abort();
    }
}
