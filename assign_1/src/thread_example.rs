use std::thread;
use std::time::Duration;

fn digital_car_start(){
	
	let start_engine = thread::spawn(move || {
		thread::sleep(Duration::from_millis(200));
		println!("Start the Engine, exhaust fires");
	});

	let start_ecu = thread::spawn(move || {
		thread::sleep(Duration::from_millis(120));
		println!("Electronic Controller Unit Started");
	});

	let start_transmission = thread::spawn(move || {
		thread::sleep(Duration::from_millis(360));
		println!("Transmission Unlocked");
	});
	
	let start_automod = thread::spawn(move || {
		thread::sleep(Duration::from_millis(540));
		println!("Auto Mode detected | Auto-pilot procedures Activated")
	});


	start_engine.join().unwrap();
	start_ecu.join().unwrap();
	start_transmission.join().unwrap();
	start_automod.join().unwrap();
}


pub fn main () {
	println!("Thread Example");
	// Thread starting the car system
	digital_car_start()
	
}