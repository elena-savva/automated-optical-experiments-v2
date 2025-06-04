use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use std::thread;
use visa_rs::prelude::*;
use crate::visa_error;
use visa_error::io_to_vs_err;

/// Runs a wavelength sweep with the specified parameters
pub fn run_wavelength_sweep(
    laser: &mut Instrument,
    power_meter: &mut Instrument,
    start_nm: f64,
    stop_nm: f64,
    step_nm: f64,
    stabilization_time_ms: u64,
) -> visa_rs::Result<()> {
    // Create a CSV file to save results
    std::fs::create_dir_all("data").unwrap_or_else(|e| {
        println!("Warning: Failed to create data directory: {}", e);
    });
    let mut file = File::create("data/wavelength_sweep_results.csv").map_err(io_to_vs_err)?;
    writeln!(file, "Wavelength (nm),Power (dBm)").map_err(io_to_vs_err)?;
    
    // Calculate number of points
    let num_points = ((stop_nm - start_nm) / step_nm).floor() as usize + 1;
    println!("Starting wavelength sweep with {} points", num_points);
    
    // Configure the laser for the experiment
    laser.write_all(b":SOURce2:POWer:STATe 0\n").map_err(io_to_vs_err)?;
    laser.write_all(b":SOURce2:WAVelength:AUTO 1\n").map_err(io_to_vs_err)?;  // Set power to default value
    laser.write_all(b":SOURce2:POWer:UNit 0\n").map_err(io_to_vs_err)?;  // Set power unit to dBm
    laser.write_all(b":SOURce2:POWer:LEVel:IMMediate:AMPLitude DEF\n").map_err(io_to_vs_err)?;  // Set power to default value

    // Configure the power meter for the experiment
    power_meter.write_all(b"WMOD CONST1\n").map_err(io_to_vs_err)?;  // Constant wavelength mode
    power_meter.write_all(b"AVG 50\n").map_err(io_to_vs_err)?;        // 50ms averaging time
    power_meter.write_all(b"UNIT 0\n").map_err(io_to_vs_err)?;        // dBm units
    
    // Turn laser 2 ON
    laser.write_all(b":SOURce2:POWer:STATe 1\n").map_err(io_to_vs_err)?;
    println!("Laser turned ON");
    
    let cmd = format!(":SOURce2:WAVelength:CW {:.3}NM\n", start_nm);
    laser.write_all(cmd.as_bytes()).map_err(io_to_vs_err)?;
    // Wait for initial stabilization
    thread::sleep(Duration::from_millis(stabilization_time_ms));
    
    // Perform the sweep
    for i in 0..num_points {
        let wavelength = start_nm + (i as f64 * step_nm);
        
        // Validate wavelength is within safe range
        if wavelength < 1527.60 || wavelength > 1570.01 {
            return Err(io_to_vs_err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Wavelength {:.2} nm is outside safe range (1527.60-1611.76 nm)", wavelength)
            )));
        }
        
        // Set the laser wavelength
        let cmd = format!(":SOURce2:WAVelength:CW {:.3}NM\n", wavelength);
        laser.write_all(cmd.as_bytes()).map_err(io_to_vs_err)?;
        
        // Update power meter wavelength calibration
        let cmd = format!("WAV {:.3}\n", wavelength);
        power_meter.write_all(cmd.as_bytes()).map_err(io_to_vs_err)?;
        
        println!("Set wavelength to {:.2} nm", wavelength);
        
        // Wait for stabilization
        thread::sleep(Duration::from_millis(stabilization_time_ms));
        
        // Measure power
        power_meter.write_all(b"READ? 0\n").map_err(io_to_vs_err)?;
        
        let mut power_response = String::new();
        {
            let mut reader = BufReader::new(&*power_meter);
            reader.read_line(&mut power_response).map_err(io_to_vs_err)?;
        }
        
        // Parse the power value (format: "power1,power2,power3,power4")
        let power_values: Vec<&str> = power_response.trim().split(',').collect();
        if power_values.is_empty() {
            return Err(io_to_vs_err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No power readings received"
            )));
        }
        
        let power = match power_values[1].parse::<f64>() { // Save power value from the 2nd port (top to bottom)
            Ok(value) => value,
            Err(_) => return Err(io_to_vs_err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse power value: {}", power_values[1])
            ))),
        };
        
        // Print measured values
        println!("  Power: {:.3} dBm", power);
        
        // Write to results file
        writeln!(file, "{:.3},{:.6}", wavelength, power).map_err(io_to_vs_err)?;
    }
    
    // Turn laser OFF
    laser.write_all(b":SOURce2:POWer:STATe 0\n").map_err(io_to_vs_err)?;
    println!("Laser turned OFF");
    
    // Check for errors on laser
    laser.write_all(b"SYST:ERR?\n").map_err(io_to_vs_err)?;
    
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&*laser);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    
    println!("Final error check on laser: {}", response.trim());
    
    // Check for errors on power meter
    power_meter.write_all(b"ERR?\n").map_err(io_to_vs_err)?;
    
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&*power_meter);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    
    println!("Final error check on power meter: {}", response.trim());
    
    println!("Wavelength sweep completed successfully");
    println!("Results saved to wavelength_sweep_results.csv");
    
    Ok(())
}
