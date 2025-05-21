# Laboratory Experiment Control System

This application provides a web-based interface for controlling laboratory experiments with multiple instruments.

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

## Setup Instructions

1. Install Rust and Cargo (if not already installed): https://www.rust-lang.org/tools/install

2. Make sure you have the appropriate VISA libraries installed for your operating system:
   - For Windows: NI-VISA or similar
   - For Linux: Check out the visa-rs crate documentation

3. Create a `frontend` directory in the project root and place the HTML file there:
   ```
   mkdir -p frontend
   # Copy the HTML file to frontend/index.html
   ```

4. Build and run the application:
   ```
   cargo build --release
   cargo run --release
   ```

5. Open the web interface in your browser:
   ```
   http://localhost:3000
   ```

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