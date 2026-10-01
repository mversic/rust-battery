//! This crate provides cross-platform information about batteries.
//!
//! Gives access to a system independent battery state, capacity, charge and voltage values
//! recalculated as necessary to be returned in [SI measurement units](https://www.bipm.org/en/measurement-units/).
//!
//! ## Supported platforms
//!
//! * Linux 2.6.39+
//! * MacOS 10.10+
//! * Windows 7+
//! * FreeBSD
//! * DragonFlyBSD
//!
//! ## Examples
//!
//! For a quick example see the [Manager](struct.Manager.html) type documentation
//! or [`simple.rs`](https://github.com/svartalf/rust-battery/blob/master/battery/examples/simple.rs)
//! file in the `examples/` folder.
//!
//! [battop](https://crates.io/crates/battop) crate is using this library as a knowledge source,
//! so check it out too for a real-life example.

#![deny(unused)]
#![deny(unstable_features)]
#![deny(bare_trait_objects)]
#![allow(clippy::manual_non_exhaustive)]  // MSRV is 1.36
#![doc(html_root_url = "https://docs.rs/battery/0.7.8")]

#[cfg(not(all(feature = "co3", not(feature = "export"))))]
#[macro_use]
extern crate cfg_if;

#[cfg(target_os = "windows")]
#[macro_use]
extern crate winapi;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
#[macro_use]
extern crate nix;

mod types;
#[macro_use]
pub mod units;
pub mod errors;
#[cfg(not(all(feature = "co3", not(feature = "export"))))]
mod platform;

#[cfg(feature = "co3")]
use core::ffi::{c_float, c_int};

pub use self::errors::{Error, Result};
pub use self::types::{State, Technology};
#[cfg(not(all(feature = "co3", not(feature = "export"))))]
pub use self::types::{Batteries, Battery, Manager};

#[cfg(feature = "export")]
pub fn battery_str_free(_value: String) {}

#[cfg(feature = "export")]
pub use errors::battery_have_last_error;

#[cfg(feature = "export")]
use errors::battery_last_error_message;

#[cfg(feature = "co3")]
co3::ffi! {
    #![cfg_attr(feature = "export", unsafe(export("C")))]
    #![cfg_attr(not(feature = "export"), unsafe(extern("C")))]

    pub type Manager;
    pub type Batteries;
    pub type Battery;

    impl Drop for Manager {
        #[symbol_name = "battery_manager_free"]
        fn drop(&mut self);
    }

    impl Drop for Batteries {
        #[symbol_name = "battery_iterator_free"]
        fn drop(&mut self);
    }

    impl Drop for Battery {
        #[symbol_name = "battery_free"]
        fn drop(&mut self);
    }

    impl Manager {
        #[symbol_name = "battery_manager_new"]
        pub fn new_ffi() -> move Option<OwnedManager>;

        #[symbol_name = "battery_manager_iter"]
        pub fn batteries_ffi(&self) -> move Option<OwnedBatteries>;

        #[symbol_name = "battery_manager_refresh"]
        pub fn refresh_ffi(&mut self, battery: &mut Battery) -> c_int;
    }

    impl Batteries {
        #[symbol_name = "battery_iterator_next"]
        pub fn next_ffi(&mut self) -> move Option<OwnedBattery>;
    }

    impl Battery {
        #[symbol_name = "battery_get_state"]
        pub fn state(&self) -> State;

        #[symbol_name = "battery_get_technology"]
        pub fn technology(&self) -> Technology;

        #[symbol_name = "battery_get_state_of_charge"]
        pub fn state_of_charge_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_energy"]
        pub fn energy_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_energy_full"]
        pub fn energy_full_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_energy_full_design"]
        pub fn energy_full_design_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_energy_rate"]
        pub fn energy_rate_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_voltage"]
        pub fn voltage_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_state_of_health"]
        pub fn state_of_health_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_time_to_full"]
        pub fn time_to_full_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_time_to_empty"]
        pub fn time_to_empty_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_temperature"]
        pub fn temperature_ffi(&self) -> c_float;

        #[symbol_name = "battery_get_cycle_count"]
        pub fn cycle_count_ffi(&self) -> u32;

        #[symbol_name = "battery_get_vendor"]
        pub fn vendor_ffi(&self) -> move Option<String>;

        #[symbol_name = "battery_get_model"]
        pub fn model_ffi(&self) -> move Option<String>;

        #[symbol_name = "battery_get_serial_number"]
        pub fn serial_number_ffi(&self) -> move Option<String>;
    }

    #[symbol_name = "battery_last_error_message"]
    pub fn battery_last_error_message() -> move Option<String>;

    #[symbol_name = "battery_have_last_error"]
    pub fn battery_have_last_error() -> c_int;

    #[symbol_name = "battery_str_free"]
    pub fn battery_str_free(value: move String);
}
