// src/web_server.rs

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::ffi::CString;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use std::path::Path;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use visa_rs::prelude::*;
use crate::cld1015_osa;
use crate::n77_wavelength_check;
use crate::n77_wavelength_sweep;
use crate::n77_osa;
use crate::visa_error::io_to_vs_err;

// Configure CORS to allow all origins
fn with_cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type"])
}

// State shared across handlers
struct AppState {
    rm: Option<DefaultRM>,
    devices: DeviceState,
}

struct DeviceState {
    cld1015: Option<Instrument>,
    n77: Option<Instrument>,
    power_meter: Option<Instrument>,
    osa: Option<Instrument>,
    cld1015_info: String,
    n77_info: String,
    power_meter_info: String,
    osa_info: String,
}

impl AppState {
    fn new() -> Self {
        AppState {
            rm: None,
            devices: DeviceState {
                cld1015: None,
                n77: None,
                power_meter: None,
                osa: None,
                cld1015_info: String::new(),
                n77_info: String::new(),
                power_meter_info: String::new(),
                osa_info: String::new(),
            },
        }
    }
}

#[derive(Serialize)]
struct ConnectionStatus {
    connected: bool,
    info: Option<String>,
    error: Option<String>,
}

#[derive(Serialize)]
struct ExperimentResult {
    success: bool,
    result_path: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct CurrentSweepParams {
    start_ma: f64,
    stop_ma: f64,
    step_ma: f64,
    dwell_time_ms: u64,
}

#[derive(Deserialize)]
struct WavelengthCheckParams {
    wavelength: f64,
    stabilization_time_ms: u64,
}

#[derive(Deserialize)]
struct WavelengthSweepParams {
    start_nm: f64,
    stop_nm: f64,
    step_nm: f64,
    stabilization_time_ms: u64,
}

// Filter to inject state
fn with_state(
    state: Arc<Mutex<AppState>>,
) -> impl Filter<Extract = (Arc<Mutex<AppState>>,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

// Device connection helper functions
fn check_cld1015(state: &mut AppState) -> std::result::Result<String, String> {
    let resource = CString::new("USB::4883::32847::M01053290::0::INSTR").unwrap();
    
    // Check if resource manager is initialized
    if state.rm.is_none() {
        state.rm = match DefaultRM::new() {
            Ok(rm) => Some(rm),
            Err(err) => return Err(format!("Failed to init VISA resource manager: {}", err)),
        };
    }
    
    let rm = state.rm.as_ref().unwrap();
    
    // Open device if not already open
    if state.devices.cld1015.is_none() {
        state.devices.cld1015 = match rm.open(
            &resource.into(),
            AccessMode::NO_LOCK,
            Duration::from_secs(1),
        ) {
            Ok(inst) => Some(inst),
            Err(err) => return Err(format!("Failed to open CLD1015: {}", err)),
        };
    }
    
    // Get the device
    let device = state.devices.cld1015.as_mut().unwrap();
    
    // Clear any errors
    if let Err(err) = device.write_all(b"*CLS\n") {
        return Err(format!("Failed to clear CLD1015: {}", err));
    }
    
    // Query device identity
    if let Err(err) = device.write_all(b"*IDN?\n") {
        return Err(format!("Failed to query CLD1015: {}", err));
    }
    
    // Read the response
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&mut *device);
        if let Err(err) = reader.read_line(&mut response) {
            return Err(format!("Failed to read CLD1015 response: {}", err));
        }
    }
    
    // Save info
    state.devices.cld1015_info = response.trim().to_string();
    Ok(response.trim().to_string())
}

fn check_n77(state: &mut AppState) -> std::result::Result<String, String> {
    let resource = CString::new("GPIB0::21::INSTR").unwrap();
    
    // Check if resource manager is initialized
    if state.rm.is_none() {
        state.rm = match DefaultRM::new() {
            Ok(rm) => Some(rm),
            Err(err) => return Err(format!("Failed to init VISA resource manager: {}", err)),
        };
    }
    
    let rm = state.rm.as_ref().unwrap();
    
    // Open device if not already open
    if state.devices.n77.is_none() {
        state.devices.n77 = match rm.open(
            &resource.into(),
            AccessMode::NO_LOCK,
            Duration::from_secs(1),
        ) {
            Ok(inst) => Some(inst),
            Err(err) => return Err(format!("Failed to open N77: {}", err)),
        };
    }
    
    // Get the device
    let device = state.devices.n77.as_mut().unwrap();
    
    // Clear any errors
    if let Err(err) = device.write_all(b"*CLS\n") {
        return Err(format!("Failed to clear N77: {}", err));
    }
    
    // Query device identity
    if let Err(err) = device.write_all(b"*IDN?\n") {
        return Err(format!("Failed to query N77: {}", err));
    }
    
    // Read the response
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&mut *device);
        if let Err(err) = reader.read_line(&mut response) {
            return Err(format!("Failed to read N77 response: {}", err));
        }
    }
    
    // Save info
    state.devices.n77_info = response.trim().to_string();
    Ok(response.trim().to_string())
}

fn check_power_meter(state: &mut AppState) -> std::result::Result<String, String> {
    let resource = CString::new("GPIB0::16::INSTR").unwrap();
    
    // Check if resource manager is initialized
    if state.rm.is_none() {
        state.rm = match DefaultRM::new() {
            Ok(rm) => Some(rm),
            Err(err) => return Err(format!("Failed to init VISA resource manager: {}", err)),
        };
    }
    
    let rm = state.rm.as_ref().unwrap();
    
    // Open device if not already open
    if state.devices.power_meter.is_none() {
        state.devices.power_meter = match rm.open(
            &resource.into(),
            AccessMode::NO_LOCK,
            Duration::from_secs(1),
        ) {
            Ok(inst) => Some(inst),
            Err(err) => return Err(format!("Failed to open Power Meter: {}", err)),
        };
    }
    
    // Get the device
    let device = state.devices.power_meter.as_mut().unwrap();
    
    // Clear any errors
    if let Err(err) = device.write_all(b"*CLS\n") {
        return Err(format!("Failed to clear Power Meter: {}", err));
    }
    
    // Query device identity
    if let Err(err) = device.write_all(b"*IDN?\n") {
        return Err(format!("Failed to query Power Meter: {}", err));
    }
    
    // Read the response
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&mut *device);
        if let Err(err) = reader.read_line(&mut response) {
            return Err(format!("Failed to read Power Meter response: {}", err));
        }
    }
    
    // Save info
    state.devices.power_meter_info = response.trim().to_string();
    Ok(response.trim().to_string())
}

fn check_osa(state: &mut AppState) -> std::result::Result<String, String> {
    let resource = CString::new("GPIB0::23::INSTR").unwrap();
    
    // Check if resource manager is initialized
    if state.rm.is_none() {
        state.rm = match DefaultRM::new() {
            Ok(rm) => Some(rm),
            Err(err) => return Err(format!("Failed to init VISA resource manager: {}", err)),
        };
    }
    
    let rm = state.rm.as_ref().unwrap();
    
    // Open device if not already open
    if state.devices.osa.is_none() {
        state.devices.osa = match rm.open(
            &resource.into(),
            AccessMode::NO_LOCK,
            Duration::from_secs(1),
        ) {
            Ok(inst) => Some(inst),
            Err(err) => return Err(format!("Failed to open OSA: {}", err)),
        };
    }
    
    // Get the device
    let device = state.devices.osa.as_mut().unwrap();
    
    // Clear any errors
    if let Err(err) = device.write_all(b"CLS;\n") {
        return Err(format!("Failed to clear OSA: {}", err));
    }
    
    // Query device identity
    if let Err(err) = device.write_all(b"ID?;\n") {
        return Err(format!("Failed to query OSA: {}", err));
    }
    
    // Read the response
    let mut response = String::new();
    {
        let mut reader = BufReader::new(&mut *device);
        if let Err(err) = reader.read_line(&mut response) {
            return Err(format!("Failed to read OSA response: {}", err));
        }
    }
    
    // Save info
    state.devices.osa_info = response.trim().to_string();
    Ok(response.trim().to_string())
}

// Handler for checking device connection
async fn check_connection_handler(
    device: String,
    state: Arc<Mutex<AppState>>,
) -> std::result::Result<impl Reply, Rejection> {
    let mut state_guard = state.lock().unwrap();
    
    let result = match device.as_str() {
        "cld1015" => check_cld1015(&mut state_guard),
        "n77" => check_n77(&mut state_guard),
        "power_meter" => check_power_meter(&mut state_guard),
        "osa" => check_osa(&mut state_guard),
        _ => Err(format!("Unknown device: {}", device)),
    };
    
    match result {
        Ok(info) => Ok(warp::reply::json(&ConnectionStatus {
            connected: true,
            info: Some(info),
            error: None,
        })),
        Err(err) => Ok(warp::reply::json(&ConnectionStatus {
            connected: false,
            info: None,
            error: Some(err),
        })),
    }
}

// Run experiment functions
fn run_current_sweep(
    state: &mut AppState,
    params: CurrentSweepParams,
) -> std::result::Result<(), String> {
    // Check if devices are connected
    if state.devices.cld1015.is_none() || state.devices.osa.is_none() {
        return Err("CLD1015 or OSA not connected".to_string());
    }
    
    // Validate parameters
    if params.start_ma < 0.0 || params.start_ma > 100.0 ||
       params.stop_ma < 0.0 || params.stop_ma > 100.0 || 
       params.stop_ma <= params.start_ma || params.step_ma <= 0.0 {
        return Err("Invalid current sweep parameters".to_string());
    }
    
    // Run experiment
    let cld1015 = state.devices.cld1015.as_mut().unwrap();
    let osa = state.devices.osa.as_mut().unwrap();
    
    cld1015_osa::run_current_sweep(
        cld1015,
        osa,
        params.start_ma,
        params.stop_ma,
        params.step_ma,
        params.dwell_time_ms,
    ).map_err(|e| format!("Experiment failed: {}", e))
}

fn run_wavelength_check(
    state: &mut AppState,
    params: WavelengthCheckParams,
) -> std::result::Result<(), String> {
    // Check if devices are connected
    if state.devices.n77.is_none() || state.devices.power_meter.is_none() {
        return Err("N77 laser or power meter not connected".to_string());
    }
    
    // Validate parameters
    if params.wavelength < 1527.60 || params.wavelength > 1570.01 {
        return Err("Invalid wavelength check parameters".to_string());
    }
    
    // Run experiment
    let n77 = state.devices.n77.as_mut().unwrap();
    let power_meter = state.devices.power_meter.as_mut().unwrap();
    
    n77_wavelength_check::run_wavelength_check(
        n77,
        power_meter,
        params.wavelength,
        params.stabilization_time_ms,
    ).map_err(|e| format!("Experiment failed: {}", e))
}

fn run_wavelength_sweep(
    state: &mut AppState,
    params: WavelengthSweepParams,
) -> std::result::Result<(), String> {
    // Check if devices are connected
    if state.devices.n77.is_none() || state.devices.power_meter.is_none() {
        return Err("N77 laser or power meter not connected".to_string());
    }
    
    // Validate parameters
    if params.start_nm < 1527.60 || params.start_nm > 1570.01 ||
       params.stop_nm < 1527.60 || params.stop_nm > 1570.01 || 
       params.stop_nm <= params.start_nm || params.step_nm <= 0.0 {
        return Err("Invalid wavelength sweep parameters".to_string());
    }
    
    // Check number of points (max 9)
    let num_points = ((params.stop_nm - params.start_nm) / params.step_nm).floor() as usize + 1;
    if num_points > 9 {
        return Err(format!("Too many data points: {}. Maximum allowed is 9.", num_points));
    }
    
    // Run experiment
    let n77 = state.devices.n77.as_mut().unwrap();
    let power_meter = state.devices.power_meter.as_mut().unwrap();
    
    n77_wavelength_sweep::run_wavelength_sweep(
        n77,
        power_meter,
        params.start_nm,
        params.stop_nm,
        params.step_nm,
        params.stabilization_time_ms,
    ).map_err(|e| format!("Experiment failed: {}", e))
}

fn run_wavelength_sweep_osa(
    state: &mut AppState,
    params: WavelengthSweepParams,
) -> std::result::Result<(), String> {
    // Check if devices are connected
    if state.devices.n77.is_none() || state.devices.osa.is_none() {
        return Err("N77 laser or OSA not connected".to_string());
    }
    
    // Validate parameters
    if params.start_nm < 1527.60 || params.start_nm > 1570.01 ||
       params.stop_nm < 1527.60 || params.stop_nm > 1570.01 || 
       params.stop_nm <= params.start_nm || params.step_nm <= 0.0 {
        return Err("Invalid wavelength sweep parameters".to_string());
    }
    
    // Check number of points (max 9)
    let num_points = ((params.stop_nm - params.start_nm) / params.step_nm).floor() as usize + 1;
    if num_points > 9 {
        return Err(format!("Too many data points: {}. Maximum allowed is 9.", num_points));
    }
    
    // Run experiment
    let n77 = state.devices.n77.as_mut().unwrap();
    let osa = state.devices.osa.as_mut().unwrap();
    
    n77_osa::run_wavelength_sweep_osa(
        n77,
        osa,
        params.start_nm,
        params.stop_nm,
        params.step_nm,
        params.stabilization_time_ms,
    ).map_err(|e| format!("Experiment failed: {}", e))
}

// Handler for running experiments
async fn run_experiment_handler(
    experiment: String, 
    body: warp::hyper::body::Bytes, 
    state: Arc<Mutex<AppState>>,
) -> std::result::Result<impl Reply, Rejection> {
    // Decode request body
    let params_json = match serde_json::from_slice(&body) {
        Ok(json) => json,
        Err(err) => {
            return Ok(warp::reply::json(&ExperimentResult {
                success: false,
                result_path: None,
                error: Some(format!("Invalid request body: {}", err)),
            }));
        }
    };
    
    // Run the experiment
    let mut state_guard = state.lock().unwrap();
    
    let result = match experiment.as_str() {
        "current_sweep" => {
            match serde_json::from_value::<CurrentSweepParams>(params_json) {
                Ok(params) => {
                    run_current_sweep(&mut state_guard, params).map(|_| {
                        "current_sweep_results.csv and trace_data/".to_string()
                    })
                },
                Err(err) => Err(format!("Invalid parameters: {}", err)),
            }
        },
        "wavelength_check" => {
            match serde_json::from_value::<WavelengthCheckParams>(params_json) {
                Ok(params) => {
                    run_wavelength_check(&mut state_guard, params).map(|_| {
                        "wavelength_check_result.csv".to_string()
                    })
                },
                Err(err) => Err(format!("Invalid parameters: {}", err)),
            }
        },
        "wavelength_sweep" => {
            match serde_json::from_value::<WavelengthSweepParams>(params_json) {
                Ok(params) => {
                    run_wavelength_sweep(&mut state_guard, params).map(|_| {
                        "wavelength_sweep_results.csv".to_string()
                    })
                },
                Err(err) => Err(format!("Invalid parameters: {}", err)),
            }
        },
        "wavelength_sweep_osa" => {
            match serde_json::from_value::<WavelengthSweepParams>(params_json) {
                Ok(params) => {
                    run_wavelength_sweep_osa(&mut state_guard, params).map(|_| {
                        "wavelength_sweep_trace_results.csv and trace_data/".to_string()
                    })
                },
                Err(err) => Err(format!("Invalid parameters: {}", err)),
            }
        },
        _ => Err(format!("Unknown experiment: {}", experiment)),
    };
    
    match result {
        Ok(path) => Ok(warp::reply::json(&ExperimentResult {
            success: true,
            result_path: Some(path),
            error: None,
        })),
        Err(err) => Ok(warp::reply::json(&ExperimentResult {
            success: false,
            result_path: None,
            error: Some(err),
        })),
    }
}

// Function to serve frontend
fn serve_frontend() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Read the HTML file - this would be your frontend from the first artifact
    let html = include_str!("../frontend/index.html");
    
    warp::path::end()
        .map(move || {
            warp::reply::html(html)
        })
}

// Start the web server
pub async fn start_server() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(AppState::new()));
    
    // Route for checking device connection
    let check_connection = warp::path!("api" / "check-connection" / String)
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(check_connection_handler);
    
    // Route for running experiments
    let run_experiment = warp::path!("api" / "run-experiment" / String)
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::bytes())
        .and(with_state(state.clone()))
        .and_then(run_experiment_handler);
    
    // Combine routes
    let routes = serve_frontend()
        .or(check_connection)
        .or(run_experiment)
        .with(with_cors());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server started at http://{}", addr);
    warp::serve(routes).run(addr).await;
    
    Ok(())
}