use std::fs::{self, File, create_dir_all};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::Duration;
use std::thread;
use visa_rs::prelude::*;
use crate::visa_error::io_to_vs_err;

/// Runs a wavelength sweep with the specified parameters
pub fn run_wavelength_sweep_osa(
    laser: &mut Instrument,
    osa: &mut Instrument,
    start_nm: f64,
    stop_nm: f64,
    step_nm: f64,
    stabilization_time_ms: u64,
) -> visa_rs::Result<()> {
    // Create a CSV file to save results
    std::fs::create_dir_all("data").unwrap_or_else(|e| {
        println!("Warning: Failed to create data directory: {}", e);
    });
    let mut file = File::create("data/wavelength_sweep_trace_results.csv").map_err(io_to_vs_err)?;
    writeln!(file, "Laser Wavelength (nm),Peak Wavelength (nm),Peak Power (dBm)").map_err(io_to_vs_err)?;
    
    // Create a directory to store trace data files
    let trace_dir = "data/wavelength_sweep_trace_data";
    create_dir_all(trace_dir).unwrap_or_else(|e| {
        println!("Warning: Failed to create trace data directory: {}", e);
    });

    // Calculate number of points
    let num_points = ((stop_nm - start_nm) / step_nm).floor() as usize + 1;
    println!("Starting wavelength sweep with {} points", num_points);
    
    // Configure the OSA for measurements
    osa.write_all(b"SNGLS;\n").map_err(io_to_vs_err)?; // Set to single sweep mode
    osa.write_all(b"CENTERWL 1549NM;SPANWL 44NM;\n").map_err(io_to_vs_err)?;

    
    let center_wl = 1549.00; // Center wavelength in nm
    let span_wl = 44.0;    // Span in nm
    let start_wl = start_nm; 
    let stop_wl = stop_nm;

    // Get number of data points in trace
    osa.write_all(b"MDS?;\n").map_err(io_to_vs_err)?;
    let mut mds_response = String::new();
    {
        let mut reader = BufReader::new(&*osa);
        reader.read_line(&mut mds_response).map_err(io_to_vs_err)?;
    }
    let num_trace_points = mds_response.trim().parse::<usize>().unwrap_or(800); // Default 800 if parsing fails
    println!("Trace has {} data points", num_trace_points);
    
    // Configure the laser for the experiment
    laser.write_all(b":SOURce2:POWer:STATe 0\n").map_err(io_to_vs_err)?;
    laser.write_all(b":SOURce2:WAVelength:AUTO 1\n").map_err(io_to_vs_err)?;  // Set power to default value
    laser.write_all(b":SOURce2:POWer:UNit 0\n").map_err(io_to_vs_err)?;  // Set power unit to dBm
    laser.write_all(b":SOURce2:POWer:LEVel:IMMediate:AMPLitude DEF\n").map_err(io_to_vs_err)?;  // Set power to default value
    
    // Turn laser ON
    laser.write_all(b":SOURce2:POWer:STATe 1\n").map_err(io_to_vs_err)?;
    println!("Laser turned ON");

    let cmd = format!(":SOURce2:WAVelength:CW {:.3}NM\n", 1530.00);
    laser.write_all(cmd.as_bytes()).map_err(io_to_vs_err)?;
    
    // Wait for initial stabilization
    thread::sleep(Duration::from_millis(stabilization_time_ms));

    // Trigger a new sweep on the OSA and confirm it's done before proceeding
        osa.write_all(b"TS;DONE?;\n").map_err(io_to_vs_err)?; // Take sweep
        let mut done_resp = String::new();
        {
            let mut reader = BufReader::new(&*osa);
            reader.read_line(&mut done_resp).map_err(io_to_vs_err)?;
        }
        if done_resp.trim() != "1" {
            println!("Warning: Sweep not confirmed complete. Response: {}", done_resp.trim());
        }
    
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

        // Wait for stabilization
        thread::sleep(Duration::from_millis(stabilization_time_ms));
        
        // Trigger a new sweep on the OSA and confirm it's done before proceeding
        osa.write_all(b"TS;DONE?;\n").map_err(io_to_vs_err)?; // Take sweep
        let mut done_resp = String::new();
        {
            let mut reader = BufReader::new(&*osa);
            reader.read_line(&mut done_resp).map_err(io_to_vs_err)?;
        }
        if done_resp.trim() != "1" {
            println!("Warning: Sweep not confirmed complete. Response: {}", done_resp.trim());
        }
        
        // Find peak
        osa.write_all(b"MKPK HI;\n").map_err(io_to_vs_err)?; // Mark highest signal level
        
        // Get peak wavelength
        osa.write_all(b"MKWL?;\n").map_err(io_to_vs_err)?;
        let mut peak_wavelength = String::new();
        {
            let mut reader = BufReader::new(&*osa);
            reader.read_line(&mut peak_wavelength).map_err(io_to_vs_err)?;
        }
        let peak_wavelength_nm = peak_wavelength.trim().parse::<f64>().unwrap_or(0.0) * 1.0e9; // Convert from meters to nm
        
        // Get peak amplitude
        osa.write_all(b"MKA?;\n").map_err(io_to_vs_err)?;
        let mut peak_power = String::new();
        {
            let mut reader = BufReader::new(&*osa);
            reader.read_line(&mut peak_power).map_err(io_to_vs_err)?;
        }
        let peak_power_dbm = peak_power.trim().parse::<f64>().unwrap_or(-100.0);
        
        // Print measured values
        println!("  Peak Wavelength: {:.3} nm", peak_wavelength_nm);
        println!("  Peak Power: {:.2} dBm", peak_power_dbm);
        
        // Write to results file
        writeln!(file, "{:.2},{:.4},{:.2}", 
                wavelength, peak_wavelength_nm, peak_power_dbm).unwrap();
        
        // Fetch the entire trace data
        println!("Retrieving trace data...");
        osa.write_all(b"TRA?;\n").map_err(io_to_vs_err)?;
        
        // Read trace data
        let mut wavelength_sweep_trace_data = String::new();
        {
            let mut reader = BufReader::new(&*osa);
            reader.read_line(&mut wavelength_sweep_trace_data).map_err(io_to_vs_err)?;
        }
        
        // Calculate wavelength array for the x-axis
        let wavelength_step = (stop_wl - start_wl) / (num_trace_points as f64 - 1.0);
        
        // Create trace data file
        let trace_filename = format!("{}/trace_{:.2}nm.csv", trace_dir, wavelength);
        let mut trace_file = File::create(&trace_filename).unwrap_or_else(|e| {
            println!("Warning: Failed to create trace file {}: {}", trace_filename, e);
            File::create("trace_data_fallback.csv").unwrap()
        });
        
        // Write header to trace file
        writeln!(trace_file, "Wavelength (nm),Power (dBm)").unwrap();
        
        // Parse and write trace data
        let values: Vec<&str> = wavelength_sweep_trace_data.trim().split(',').collect();
        for (j, value) in values.iter().enumerate() {
            if j < num_trace_points {
                let wavelength = start_wl + (j as f64 * wavelength_step);
                let power = value.parse::<f64>().unwrap_or(-100.0);
                writeln!(trace_file, "{:.4},{:.4}", wavelength, power).unwrap();
            }
        }
        
        println!("  Trace data saved to {}", trace_filename);
    }
    
    // Turn laser OFF
    laser.write_all(b":SOURce2:POWer:STATe 0\n").map_err(io_to_vs_err)?;
    println!("Laser turned OFF");

    osa.write_all(b"SWEEP OFF;\n").map_err(io_to_vs_err)?; // Turn off

    
    // Check for errors on laser
    laser.write_all(b"SYST:ERR?\n").map_err(io_to_vs_err)?;
    
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&*laser);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    
    println!("Final error check on laser: {}", response.trim());
    
    // Check for errors on OSA
    osa.write_all(b"XERR?;\n").map_err(io_to_vs_err)?;
    
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&*osa);
        reader.read_line(&mut response).map_err(io_to_vs_err)?;
    }
    
    println!("Final error check on OSA: {}", response.trim());
    
    println!("Wavelength sweep completed successfully");
    println!("Results saved to wavelength_sweep_trace_results.csv");
    println!("Trace data saved to {}/trace_*nm.csv files", trace_dir);

    Ok(())
}