# PNN Lab Software Suite

**A web-based interface for controlling optical laboratory instruments.**

## Quick Start for Lab Users

### Windows Users (Recommended)
**Just want to control your lab equipment? No coding required!**

1. Go to the [Releases page](https://github.com/elena-savva/automated-optical-experiments-v2/releases)
2. Download `lab-control-system-v1.0.0-windows.zip` 
3. Extract to any folder on your lab computer
4. Double-click `run_lab_control.bat`
5. Open http://localhost:3000 in your browser
6. Start experimenting! 

### Developers / Windows / Linux Users
**Build from source (requires technical setup):**

1. Go to the [Releases page](https://github.com/elena-savva/automated-optical-experiments-v2/releases)
2. Download `Source code (tar.gz)` or `Source code (zip)`
3. Extract the files
4. Install dependencies:
   - **Windows**: Install VISA drivers and GPIB drivers (NI-VISA and NI-488.2 recommended)
   - **Linux**: `sudo apt install libvisa-dev python3-pyvisa libusb-1.0-0-dev pkg-config libssl-dev`
5. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && source ~/.cargo/env`
6. Build: `cd automated-optical-experiments-v2-1.0.0 && cargo build --release`
7. Run: `./target/release/automating_experiments`
8. Open http://localhost:3000 in your browser

**Note:** Linux support is experimental.

---

## Supported Instruments

- **CLD1015 Laser Diode** (USB connection)
- **N7714A Tunable Laser** (GPIB address 21) - **Uses Laser 2 output**
- **MPM210-H Power Meter** (GPIB address 16) - **Uses Module 0, Port 2** 
- **HP-70952B Optical Spectrum Analyzer** (GPIB address 23)

### Important Hardware Configuration Notes
- **N7714A**: The software is hardcoded to control **Laser 2**. Ensure your fiber is connected to the **Laser 2 output**.
- **MPM210-H**: The software reads from **Module 0, Port 2**. Connect your optical input to **port 2** (second port from top).

## Available Experiments

- **Current Sweep** (CLD1015 Laser Diode + HP-70952B Optical Spectrum Analyzer) - Characterize laser output vs current
- **Wavelength Check** (N7714A Tunable Laser + MPM210-H Power Meter) - Single wavelength measurement
- **Wavelength Sweep** (N7714A Tunable Laser + MPM210-H Power Meter) - Power vs wavelength scan
- **Wavelength Sweep with OSA** (N7714A Tunable Laser + HP-70952B Optical Spectrum Analyzer) - Full spectral analysis

## System Requirements

### Windows
- Windows 10/11 (64-bit)
- VISA drivers (NI-VISA recommended or equivalent)
- GPIB drivers (NI-488.2 recommended or equivalent)
- Modern web browser
- Laboratory instruments connected via USB/GPIB

### Linux  
- Ubuntu 20.04+ (recommended) or Debian 10+
- VISA development libraries
- Linux GPIB package
- sudo access for installation
- Instruments connected via USB/GPIB

---

## For Developers

### Prerequisites
- Rust and Cargo installed
- VISA drivers for your operating system
- GPIB drivers for GPIB-connected instruments
- Laboratory instruments connected

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/elena-savva/automated-optical-experiments-v2.git
   cd automated-optical-experiments-v2
   ```

2. Install dependencies:
   - **Windows**: Install VISA and GPIB drivers from National Instruments
   - **Linux**: `sudo apt install libvisa-dev python3-pyvisa libusb-1.0-0-dev pkg-config libssl-dev`

3. Build and run:
   ```bash
   cargo build --release
   cargo run
   ```

4. Open the web interface: http://localhost:3000

### Creating Windows Deployment Package

```bash
# Build release version
cargo build --release

# Create deployment package
.\create_deployment.bat
```

This creates a `lab-control-deployment/` folder with everything needed for distribution.

---

## Using the Application

1. **Connect Instruments**: Ensure all laboratory instruments are powered on and connected

2. **Start the Server**: 
   - Windows: Run `run_lab_control.bat`
   - Linux: Run `./target/release/automating_experiments`

3. **Open Web Interface**: Navigate to http://localhost:3000

4. **Check Device Connections**: Click "Check Connection" for each instrument

5. **Run Experiments**:
   - Select experiment type from dropdown
   - Configure parameters (automatically validated)
   - Click "Run Experiment"
   - Results are saved as CSV files in the `data/` folder

## Experiment Parameters

| Experiment | Device Combo | Parameters |
|------------|--------------|------------|
| **Current Sweep** | CLD1015 + OSA | Start/Stop Current (0-100 mA), Step Size |
| **Wavelength Check** | N77 + Power Meter | Wavelength (1527.60-1570.01 nm) |
| **Wavelength Sweep** | N77 + Power Meter | Start/Stop Wavelength, Step Size |
| **Wavelength Sweep + OSA** | N77 + OSA | Start/Stop Wavelength, Step Size |

**Note:** Wavelength sweeps limited to 9 data points maximum.

## Data Output

All results saved to `data/` folder:
- **Summary files**: `*.csv` with key measurements
- **Trace data**: `*_trace_data/` folders with detailed spectral information

## Safety Features

- **Parameter validation** with real-time feedback
- **Safe operating ranges** enforced for all instruments
- **Error handling** with detailed diagnostic messages
- **Automatic instrument shutdown** after experiments
- **Connection verification** before experiment start

## Project Structure

```
src/
├── main.rs                  # Main entry point & web server
├── web_server.rs            # Web API and frontend serving
├── cld1015_osa.rs           # Current sweep experiments
├── n77_wavelength_check.rs  # Single wavelength measurements
├── n77_wavelength_sweep.rs  # Wavelength sweep experiments  
├── n77_osa.rs               # Wavelength sweep with OSA
└── visa_error.rs            # VISA error handling

frontend/
└── index.html               # Web interface

data/                        # Generated experiment results
├── *.csv                    # Summary data
└── *_trace_data/            # Detailed trace files
```

## Troubleshooting

### Common Issues
- **Instruments won't connect**: Check VISA and GPIB drivers and instrument power/connections
- **Web interface won't load**: Ensure no other software is using port 3000
- **Permission errors (Linux)**: Add user to dialout group: `sudo usermod -a -G dialout $USER`

### Getting Help
- Check the instrument address settings in the source code
- Verify VISA installation: try connecting with other VISA software
- For build issues: ensure all development dependencies are installed

## License

MIT

---

**Developed by Elena Savva for PNN Group, Department of Electrical Engineering, Eindhoven University of Technology**
