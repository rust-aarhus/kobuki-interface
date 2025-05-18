use anyhow::Result;
use kobuki_interface::{
    rx::{Feedback, IrSignal},
    serial_port::SerialPortHandler,
    tx::{ByteStream, commands},
};
use std::time::Duration;
use tokio::time::Instant;
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("Looking for IR signals...");

    let port = tokio_serial::new("/dev/kobuki", 115200)
        .timeout(Duration::from_millis(1024))
        .open_native_async()?;
    let serial = SerialPortHandler::new(port);

    let mut rx = serial.subscribe();
    let mut last_base_ctrl = Instant::now();
    let mut last_docking_ir = None;

    // Notice that the base control needs to be set regularly to keep the robot moving
    loop {
        tokio::select! {
            feedback = rx.recv() => {
                let feedback = feedback?;
                if feedback.docking_ir != last_docking_ir {
                    handle_feedback(&feedback);
                    last_docking_ir = feedback.docking_ir;
                }
            }
            _ = tokio::time::sleep_until(last_base_ctrl + Duration::from_secs(1)) => {
                last_base_ctrl = Instant::now();
                serial
                    .send_command(ByteStream::builder().subpayload(commands::BaseControl::new(70, 1)))
                    .await?;
            }
        }
    }
}

fn handle_feedback(feedback: &Feedback) {
    let angle = feedback
        .inertial_sensor
        .as_ref()
        .map(|s| s.angle)
        .unwrap_or(0.0);
    if let Some(docking_ir) = &feedback.docking_ir {
        println!(
            "Docking IR: {:4.0} | {} | {} | {}",
            angle,
            format_ir(docking_ir.left),
            format_ir(docking_ir.center),
            format_ir(docking_ir.right)
        );
    }
}

fn format_ir(ir: IrSignal) -> String {
    let mut result = String::new();
    if ir.contains(IrSignal::NEAR_LEFT) {
        result.push('N');
    } else {
        result.push(' ');
    }
    if ir.contains(IrSignal::FAR_LEFT) {
        result.push('F');
    } else {
        result.push(' ');
    }
    result.push_str("  ");
    if ir.contains(IrSignal::NEAR_CENTER) {
        result.push('N');
    } else {
        result.push(' ');
    }
    if ir.contains(IrSignal::FAR_CENTER) {
        result.push('F');
    } else {
        result.push(' ');
    }
    result.push_str("  ");
    if ir.contains(IrSignal::NEAR_RIGHT) {
        result.push('N');
    } else {
        result.push(' ');
    }
    if ir.contains(IrSignal::FAR_RIGHT) {
        result.push('F');
    } else {
        result.push(' ');
    }
    result
}
