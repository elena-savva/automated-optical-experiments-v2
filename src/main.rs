#![allow(unused)]

mod cld1015_osa;
mod n77_wavelength_sweep;
mod n77_wavelength_check;
mod n77_osa;
mod visa_error;
mod web_server;

use std::ffi::CString;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use visa_rs::prelude::*;
use visa_error::io_to_vs_err;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Starting experiment control server...");
    
    // Start the web server
    web_server::start_server().await?;
    
    Ok(())
}