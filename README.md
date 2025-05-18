# Rust Kobuki Interface
[![Rust](https://github.com/rust-aarhus/kobuki-interface/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rust-aarhus/kobuki-interface/actions/workflows/rust.yml)

This crate implements a Rust interface to the Kobuki robot base. 

The protocol for the robot is described in the [appendix of the Kobuki driver documentation](https://yujinrobot.github.io/kobuki/enAppendixProtocolSpecification.html).


## Running the Examples

The computer running the example needs to be connected to the Kobuki base using a USB cable. The Kobuki base should be powered on. To make it easier to find the USB device, you can add the following udev rule to your host system.

`/etc/udev/rules.d/60-kobuki.rules`:
```bash
SUBSYSTEM=="tty", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", ATTRS{serial}=="kobuki*", ATTR{device/latency_timer}="1", MODE:="0666", GROUP:="dialout", SYMLINK+="kobuki"
```

Run the example with the following command:
```bash
cargo run --example simple_drive
```

When the connection is established, you should hear the Kobuki base play a sound and the base should start moving for a few seconds.


## Cross Compiliing for AARCH64

The easiest way to cross compile for the Kobuki base is to use the `cross` crate:

```bash
cross build --target aarch64-unknown-linux-musl --example simple_drive
```


## Known Issues of the Rust Aarhus Bot

### Docking IR

The robot has three IR sensors. One at the front and two offset ~45 degrees to the left and right. The IR sensors are used to detect the docking station. 

The docking station is emitting three IR signals. One narrow signal for the center, and wide signals for the left and right side.

Each of the three sensors on the robot is able to detect the three signals from the docking station. 

The robot should also be able to detect if the docking station is far or near.

![Docking IR](https://yujinrobot.github.io/kobuki/dock_ir_fields.png)

Two issues have been observed with the IR sensors:

1. The narrow central signal from the docking station, is actually spread out over a wide angle, and seems to be more or less useless for docking. However the left and right signals can be observed at the same time, indicating that the sensor is in front of the docking station.
2. When the robot is getting close to the docking station, the near signals are detected, but so is the far signals.

If you want to test the IR sensors, checkout the example `docker_ir.rs`.


## Credits

This is inspired by the older crate [turtlebot2](https://crates.io/crates/turtlebot2).
