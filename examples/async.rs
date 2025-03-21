use futures::StreamExt;
use sensehat_stick::JoyStick;

fn main() {
    // Open the joystick device.
    let stick = JoyStick::open().unwrap();

    // Run an async block on the current thread that polls the stream.
    futures::executor::block_on(async {
        // Since JoyStick implements Stream, we can use .next() to get events.
        let mut stick = stick; // mutable binding
        while let Some(event) = stick.next().await {
            match event {
                Ok(ev) => println!("{:?}", ev),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    });
}

