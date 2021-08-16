#![allow(unused_imports)]
// reference: https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
use futures::prelude::*;
use tokio::prelude::*;
use std::thread;
use std::time::Duration;

async fn start_engine() {
	thread::sleep(Duration::from_millis(200));
	println!("Start the Engine, exhaust fires");
}

async fn start_ecu () {
	thread::sleep(Duration::from_millis(120));
	println!("Electronic Controller Unit Started");
}

async fn start_transmission () {
  start_engine().await;
	thread::sleep(Duration::from_millis(360));
	println!("Transmission Unlocked");
}
	
async fn start_automod () {
  start_ecu().await;
	thread::sleep(Duration::from_millis(540));
	println!("Auto Mode detected | Auto-pilot procedures Activated")
}



#[tokio::main]
pub async fn main() {
    println!("Async Await Example");
    println!("--------------------");
    // start_engine().await;
    // start_ecu().await;
    start_transmission().await;
    start_automod().await;
}
