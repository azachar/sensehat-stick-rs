//! A Rust library for the Raspberry Pi Sense HAT Joystick.
//! =======================================================
//!
//! This library supports the joystick incorporated with the Sense HAT.
//!
//! The Joystick provides a driver for `evdev`.

use evdev::{Device, EventStream};
use futures_core::{ready, Stream};
use glob::glob;
use num_enum::TryFromPrimitive;

use std::io;
use std::task::Poll;
use std::time::{Duration, UNIX_EPOCH};

// Device name provided by the hardware. We match against it.
const SENSE_HAT_EVDEV_NAME: &[u8; 31] = b"Raspberry Pi Sense HAT Joystick";

/// Direction in which the JoyStick is moved.
///
/// Internally, it matches the key-press events:
///
/// * `Direction::Enter = 28`
/// * `Direction::Up = 103`
/// * `Direction::Down = 108`
/// * `Direction::Left = 105`
/// * `Direction::Up = 106`
#[repr(usize)]
#[derive(Debug, TryFromPrimitive)]
pub enum Direction {
    Enter = 28,
    Up = 103,
    Down = 108,
    Left = 105,
    Right = 106,
}

/// The action that was executed with the given `Direction`.
#[repr(usize)]
#[derive(Debug, TryFromPrimitive)]
pub enum Action {
    Release = 0,
    Press = 1,
    Hold = 2,
}

/// An event issued by the `JoyStick`. Provides a UNIX-timestamp in the form of
/// `std::time::Duration`, the `Direction`, and the `Action` that were issued by the `JoyStick`.
#[derive(Debug)]
pub struct JoyStickEvent {
    pub timestamp: Duration,
    pub direction: Direction,
    pub action: Action,
}

impl JoyStickEvent {
    fn new(timestamp: Duration, direction: Direction, action: Action) -> Self {
        JoyStickEvent {
            timestamp,
            direction,
            action,
        }
    }
}

/// A type representing the Sense HAT joystick device.
#[pin_project::pin_project]
pub struct JoyStick {
    #[pin]
    device: EventStream,
}

impl Stream for JoyStick {
    type Item = io::Result<JoyStickEvent>;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            let key = match ready!(this.device.poll_event(cx)) {
                Ok(key) => key,
                Err(e) => return Poll::Ready(Some(Err(e))),
            };

            if key.event_type().0 != 1 {
                continue;
            }

            let time = key.timestamp().duration_since(UNIX_EPOCH).unwrap();

            let direction = Direction::try_from(key.code() as usize).unwrap();
            let action = Action::try_from(key.value() as usize).unwrap();
            return Poll::Ready(Some(Ok(JoyStickEvent::new(time, direction, action))));
        }
    }
}

impl JoyStick {
    /// Open the joystick device by name in the `/dev/input/event*` path on the filesystem.
    pub fn open() -> Result<Self, io::Error> {
        for entry in glob("/dev/input/event*")
            .map_err(|e| std::io::Error::new(io::ErrorKind::InvalidInput, e))?
        {
            match entry {
                Ok(path) => {
                    let device = Device::open(&path)?;
                    if device.name().unwrap_or_default().as_bytes() == SENSE_HAT_EVDEV_NAME {
                        return Ok(JoyStick {
                            device: device.into_event_stream()?,
                        });
                    }
                }
                Err(e) => return Err(e.into_error()),
            }
        }
        return Err(std::io::Error::new(
            io::ErrorKind::NotFound,
            "No Joystick found",
        ));
    }
}
