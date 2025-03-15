# Rust Kobuki Interface
[![Rust](https://github.com/rust-aarhus/kobuki-interface/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rust-aarhus/kobuki-interface/actions/workflows/rust.yml)

This crate implements a Rust interface to the Kobuki robot base. 

The protocol for the robot is described in the [appendix of the Kobuki driver documentation](https://yujinrobot.github.io/kobuki/enAppendixProtocolSpecification.html).


## Running the example

The computer running the example needs to be connected to the Kobuki base using a USB cable. The Kobuki base should be powered on. To make it easier to find the USB device, you can add the following udev rule to your host system.

`/etc/udev/rules.d/60-kobuki.rules`:
```bash
SUBSYSTEM=="tty", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", ATTRS{serial}=="kobuki*", ATTR{device/latency_timer}="1", MODE:="0666", GROUP:="dialout", SYMLINK+="kobuki"
```

Run the example with the following command:
```bash
cargo run --example simple_drive
```

When the connection is established, you should hear the Kobuki base play a sound and the base should start moving for a couple of seconds.


## Credits

This is inspired by the older crate [turtlebot2](https://crates.io/crates/turtlebot2).
