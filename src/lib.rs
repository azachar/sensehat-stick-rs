//! A Rust library for the Raspberry Pi Sense HAT Joystick.
//! =======================================================
//!
//! This library supports the joystick incorporated with the Sense HAT.
//!
//! The Joystick provides a driver for `evdev`.
#[cfg(feature = "linux-evdev")]
extern crate evdev;
extern crate glob;

use evdev::Device;
use glob::glob;
use num_enum::TryFromPrimitive;

use std::io;
use std::os::{fd::AsRawFd, unix::io::RawFd};
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
pub struct JoyStick {
    device: Device,
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
                        return Ok(JoyStick { device });
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

    /// Returns a result with a `Vec<JoyStickEvent>`. This function will
    /// block the current thread until events are issued by the `JoyStick` device.
    pub fn events(&mut self) -> io::Result<Vec<JoyStickEvent>> {
        let events: Vec<JoyStickEvent> = self
            .device
            .fetch_events()
            .map_err(|e| io::Error::from(e))?
            .filter(|ev| ev.event_type().0 == 1)
            .map(|ev| {
                let time = ev.timestamp().duration_since(UNIX_EPOCH).unwrap();

                let direction = Direction::try_from(ev.code() as usize).unwrap();
                let action = Action::try_from(ev.value() as usize).unwrap();
                JoyStickEvent::new(time, direction, action)
            })
            .collect();
        Ok(events)
    }

    /// Returns the raw file-descriptor, `RawFd`, for the the Joystick.
    pub fn fd(&self) -> RawFd {
        self.device.as_raw_fd()
    }
}
