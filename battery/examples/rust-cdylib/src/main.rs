use battery::{battery_have_last_error, battery_last_error_message, Manager};
use std::io;

fn last_error() -> io::Error {
    let message = battery_last_error_message()
        .unwrap_or_else(|| "unknown battery error".to_owned());
    io::Error::new(io::ErrorKind::Other, message)
}

fn main() -> io::Result<()> {
    let manager = Manager::new_ffi().ok_or_else(last_error)?;
    let mut batteries = manager.batteries_ffi().ok_or_else(last_error)?;

    loop {
        let battery = match batteries.next_ffi() {
            Some(battery) => battery,
            None if battery_have_last_error() != 0 => return Err(last_error()),
            None => break,
        };

        println!(
            "{:?}: {:?}, {:.1}%",
            battery.technology(),
            battery.state(),
            battery.state_of_charge_ffi(),
        );
    }

    Ok(())
}
