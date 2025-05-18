use crate::{
    rx::{Feedback, FeedbackDecoder},
    tx::{ByteStream, commands},
};
use log::error;
use std::io::ErrorKind;
use tokio::{
    io::AsyncReadExt,
    signal,
    sync::{broadcast, mpsc},
    task::JoinHandle,
};
use tokio_serial::SerialStream;
use tokio_util::codec::Decoder;

pub struct SerialPortHandler {
    cmd_tx: mpsc::Sender<ByteStream>,
    feedback_rx: broadcast::Receiver<Feedback>,
    _serial_task: SerialPortTask,
}

impl SerialPortHandler {
    pub fn new(port: SerialStream) -> Self {
        let (feedback_tx, feedback_rx) = broadcast::channel(10);
        let (cmd_tx, cmd_rx) = mpsc::channel(10);
        let serial_task = SerialPortTask::new(port, cmd_rx, feedback_tx);
        Self {
            cmd_tx,
            feedback_rx,
            _serial_task: serial_task,
        }
    }

    pub async fn send_command(&self, cmd: ByteStream) -> std::io::Result<()> {
        self.cmd_tx
            .send(cmd)
            .await
            .map_err(|_| std::io::Error::new(ErrorKind::BrokenPipe, "mpsc channel closed"))?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Feedback> {
        self.feedback_rx.resubscribe()
    }
}

struct SerialPortTask {
    task: JoinHandle<()>,
}

impl SerialPortTask {
    fn new(
        port: SerialStream,
        cmd_rx: mpsc::Receiver<ByteStream>,
        feedback_tx: broadcast::Sender<Feedback>,
    ) -> Self {
        let task = tokio::spawn(async move {
            if let Err(e) = Self::run(port, cmd_rx, feedback_tx).await {
                error!("Error handling serial port: {:?}", e);
            }
        });

        Self { task }
    }

    pub async fn run(
        mut port: SerialStream,
        mut cmd_rx: mpsc::Receiver<ByteStream>,
        feedback_tx: broadcast::Sender<Feedback>,
    ) -> std::io::Result<()> {
        let mut decoder = FeedbackDecoder {};
        let mut buf = bytes::BytesMut::new();

        loop {
            tokio::select! {
                cmd = cmd_rx.recv() => {
                    Self::handle_command(cmd, &mut port).await?;
                }
                size = port.read_buf(&mut buf) => {
                    Self::handle_read(size?, &mut buf, &mut decoder, &feedback_tx)?;
                }
                _ = signal::ctrl_c() => {
                    break;
                }
            }
        }

        let stop_cmd = ByteStream::builder().subpayload(commands::BaseControl::new(0, 0));
        Self::handle_command(Some(stop_cmd), &mut port).await?;
        Ok(())
    }

    async fn handle_command(
        cmd: Option<ByteStream>,
        port: &mut SerialStream,
    ) -> std::io::Result<()> {
        match cmd {
            Some(bs) => {
                port.writable().await?;
                let data = bs.to_bytes();
                let _ = port.try_write(&data)?;
            }
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    "Connection closed by peer",
                ));
            }
        }
        Ok(())
    }

    fn handle_read(
        size: usize,
        buf: &mut bytes::BytesMut,
        decoder: &mut FeedbackDecoder,
        feedback_tx: &broadcast::Sender<Feedback>,
    ) -> std::io::Result<()> {
        if size == 0 {
            while let Some(frame) = decoder.decode_eof(buf)? {
                feedback_tx.send(frame).ok(); // send will give error if no subscribers - ignore errors
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "Connection closed by peer",
            ));
        }

        while let Some(frame) = decoder.decode(buf)? {
            feedback_tx.send(frame).ok(); // send will give error if no subscribers - ignore errors
        }
        Ok(())
    }
}

impl Drop for SerialPortTask {
    fn drop(&mut self) {
        self.task.abort();
    }
}
