# Laboratory Experiment Control System

This application provides a web-based interface for controlling laboratory experiments with multiple instruments.

## Quick Start for Lab Users

**Just want to control your lab equipment? No coding required!**

### [📥 Download Latest Release](https://github.com/elena-savva/automated-optical-experiments-v2/releases/tag/v1.0.0)

1. Download `lab-control-system-v1.0.0-windows.zip` 
2. Extract to any folder
3. Run `run_lab_control.bat`
4. Open http://localhost:3000 in your browser
5. Start experimenting! 🔬


## For Lab Users (Non-Developers)

If you just want to run the lab equipment, **download the pre-built release** from the [Releases page](https://github.com/elena-savva/automated-optical-experiments-v2/releases/tag/v1.0.0) instead of building from source.

## For Developers

### Prerequisites
- Rust and Cargo installed
- VISA drivers for your operating system
- Laboratory instruments (CLD1015, N7714A, MPM210-H, HP-70952B)

### Building and Running

1. Clone the repository:
   ```bash
   git clone https://github.com/elena-savva/automated-optical-experiments-v2.git
   cd automating_experiments
   ```

3. Build and run the application:
   ```
   cargo build
   cargo run
   ```

4. Open the web interface in your browser:
   ```
   http://localhost:3000
   ```
   
## Features

- Connect to and monitor 4 laboratory devices:
  - CLD1015 Laser Diode
  - N7714A Tunable Laser
  - MPM210-H Power Meter
  - HP-70952B Optical Spectrum Analyzer (OSA)
- Run 4 different types of experiments:
  - Current Sweep (CLD1015 + OSA)
  - Wavelength Check (N77 + Power Meter)
  - Wavelength Sweep (N77 + Power Meter)
  - Wavelength Sweep with OSA (N77 + OSA)
- Set experiment parameters with validation
- View experiment results in CSV files

## Using the Application

1. Check device connections:
   - Click the "Check Connection" button for each device you want to use
   - The status indicator will turn green if connected, red if disconnected

2. Select an experiment from the dropdown

3. Enter experiment parameters:
   - Parameters are automatically validated
   - For wavelength sweeps, the maximum number of readings is limited to 9

4. Click "Run Experiment" to start the experiment
   - You need to have the required devices connected for the selected experiment
   - After completion, a notification will show where the results are stored

5. Find experiment results in the generated CSV files and trace data directory

## Experiment Types and Parameters

- **Current Sweep** (CLD1015 + OSA)
  - Start Current (0-100 mA)
  - Stop Current (0-100 mA)
  - Step Size (mA)
  - Dwell Time (ms)

- **Wavelength Check** (N77 + Power Meter)
  - Wavelength (1527.60-1570.01 nm)
  - Stabilization Time (ms)

- **Wavelength Sweep** (N77 + Power Meter)
  - Start Wavelength (1527.60-1570.01 nm)
  - Stop Wavelength (1527.60-1570.01 nm)
  - Step Size (nm)
  - Stabilization Time (ms)

- **Wavelength Sweep with OSA** (N77 + OSA)
  - Start Wavelength (1527.60-1570.01 nm)
  - Stop Wavelength (1527.60-1570.01 nm)
  - Step Size (nm)
  - Stabilization Time (ms)

## Project Structure

- `src/main.rs` - Main entry point
- `src/web_server.rs` - Web server implementation
- `src/cld1015_osa.rs` - Current sweep experiment
- `src/n77_wavelength_check.rs` - Wavelength check experiment
- `src/n77_wavelength_sweep.rs` - Wavelength sweep experiment
- `src/n77_osa.rs` - Wavelength sweep with OSA experiment
- `src/visa_error.rs` - Error handling for VISA operations
- `frontend/index.html` - Web interface
