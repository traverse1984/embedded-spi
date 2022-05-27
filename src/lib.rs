#![no_std]
#![warn(clippy::all)]

pub(crate) use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin, spi::Polarity};

#[macro_use]
mod common;

mod error;
mod traits;

#[derive(Debug, Default)]
pub struct Spi;

#[cfg(feature = "hal")]
mod hal;

#[cfg(feature = "rppal")]
mod rppal;

#[cfg(feature = "rp2040")]
mod rp2040;

pub use {
    error::{Error, Result},
    traits::{ChipSelect, ClockSpeed, SpiDev},
};
