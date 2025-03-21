use futures::StreamExt;
use sensehat_stick::JoyStick;

#[tokio::main]
async fn main() {
    let mut stick = JoyStick::open().unwrap();
    while let Some(event) = stick.next().await {
        match event {
            Ok(ev) => println!("{:?}", ev),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
