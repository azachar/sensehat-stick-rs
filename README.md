A Rust library for the Raspberry Pi Sense HAT Joystick
=====================================================

[![crates.io](https://img.shields.io/crates/v/sensehat-stick.svg)](https://crates.io/crates/sensehat-stick)
[![docs](https://docs.rs/sensehat-stick/badge.svg)](https://docs.rs/sensehat-stick)

This library provides a high-level, asynchronous API for interacting with the joystick found on the [Raspberry Pi Sense HAT](https://www.raspberrypi.org/products/sense-hat/).
The Sense HAT hardware exposes a Linux `evdev` interface for the joystick so that key-events can be read directly from the device file.

Using Rust’s asynchronous features, the library leverages the futures API and can be used seamlessly within Tokio or any executor that supports Futures. (Support for synchronous polls and legacy features has been dropped.)

Installation
------------

To use this library with its default asynchronous features, add the following to your Cargo.toml:

  [dependencies]
  sensehat-stick = { git = "https://github.com/azachar/sensehat-stick-rs.git", branch="async" }
  futures = "0.3.31"
  # and optional tokio
  tokio = { version = "1.44.1", features = ["macros", "rt-multi-thread"] }

Note: The library now solely supports asynchronous operation and integrates directly with Tokio and the futures ecosystem.

Usage
-----

Because the JoyStick implements the Stream trait, you can await joystick events in an async context. The following examples show how to use the library within an asynchronous executor:

Example using Tokio:
─────────────────────

  use futures::StreamExt;
  use sensehat_stick::JoyStick;

  #[tokio::main]
  async fn main() {
      let mut stick = JoyStick::open().expect("Failed to open joystick");
      while let Some(event) = stick.next().await {
          match event {
              Ok(ev) => println!("{:?}", ev),
              Err(e) => eprintln!("Error: {:?}", e),
          }
      }
  }

Example using the Futures executor:
─────────────────────────────────────

  use futures::StreamExt;
  use sensehat_stick::JoyStick;

  fn main() {
      let stick = JoyStick::open().expect("Failed to open joystick");

      // Run an async block on the current thread using futures' executor.
      futures::executor::block_on(async {
          let mut stick = stick;
          while let Some(event) = stick.next().await {
              match event {
                  Ok(ev) => println!("{:?}", ev),
                  Err(e) => eprintln!("Error: {:?}", e),
              }
          }
      });
  }

Library Details
---------------

The library exposes a strongly-typed API for handling joystick events:

• JoyStickEvent – Wraps the event details including:
  - timestamp (UNIX duration)
  - direction (one of the Direction enum values: Enter, Up, Down, Left, or Right)
  - action (an Action enum of Press, Release, or Hold)

• JoyStick – Represents the Sense HAT Joystick. Open the device with JoyStick::open() to obtain an asynchronous stream of events.

Development and Contributions
-----------------------------

For additional examples and more detailed information on the API, please check out the [examples](./examples/) directory in the repository.
