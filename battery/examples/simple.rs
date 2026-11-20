extern crate battery;

use std::io;
use std::thread;
use std::time::Duration;

#[cfg(not(feature = "co3"))]
fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;

    loop {
        let mut batteries = manager.batteries()?;
        let battery = batteries
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Unable to find any batteries"))??;
        let charge = battery.state_of_charge().get::<battery::units::ratio::percent>();

        println!(
            "State: {}, technology: {}, charge: {:.1}%",
            battery.state(),
            battery.technology(),
            charge,
        );
        thread::sleep(Duration::from_secs(1));
    }
}

#[cfg(feature = "co3")]
fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Unable to create battery manager"))?;

    loop {
        let mut batteries = manager
            .batteries()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Unable to enumerate batteries"))?;
        let battery = batteries
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Unable to find any batteries"))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Unable to access battery information"))?;
        let charge = f32::from(battery.state_of_charge());

        println!(
            "State: {}, technology: {}, charge: {:.1}%",
            battery.state(),
            battery.technology(),
            charge,
        );
        thread::sleep(Duration::from_secs(1));
    }
}
