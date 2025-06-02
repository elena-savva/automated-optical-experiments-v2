# Laboratory Control System - Linux Installation

## Quick Start for Linux Users

### System Requirements
- Linux (Ubuntu 18.04+, Debian 10+, CentOS 8+, or equivalent)
- Internet connection
- sudo privileges

### Installation Script (Recommended)

Save this as `install_and_run.sh` and run it:

```bash
#!/bin/bash
echo "Laboratory Experiment Control System - Linux Setup"
echo "================================================="
echo

# Install VISA drivers
echo "Installing VISA drivers..."
sudo apt update
sudo apt install -y libvisa-dev python3-pyvisa libusb-1.0-0-dev pkg-config libssl-dev

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Create data directory
mkdir -p data

# Build the application
echo "Building Lab Control System (this may take a few minutes)..."
cargo build --release

if [ $? -eq 0 ]; then
    echo
    echo "Build successful! Starting Lab Control System..."
    echo "Web interface will be available at: http://localhost:3000"
    echo "Press Ctrl+C to stop the server"
    echo
    ./target/release/automating_experiments
else
    echo "Build failed. Please check the error messages above."
    exit 1
fi
```

### Manual Installation

1. **Install dependencies:**
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install -y libvisa-dev python3-pyvisa libusb-1.0-0-dev pkg-config libssl-dev

   # CentOS/RHEL/Fedora
   sudo yum install -y libvisa-devel python3-pyvisa libusb-devel pkgconfig openssl-devel
   # or: sudo dnf install -y libvisa-devel python3-pyvisa libusb-devel pkgconfig openssl-devel
   ```

2. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **Build and run:**
   ```bash
   cargo build --release
   ./target/release/automating_experiments
   ```

4. **Access the web interface:**
   Open http://localhost:3000 in your browser

## Supported Instruments

- CLD1015 Laser Diode (USB connection)
- N7714A Tunable Laser (GPIB address 21)  
- MPM210-H Power Meter (GPIB address 16)
- HP-70952B Optical Spectrum Analyzer (GPIB address 23)

## Troubleshooting

### Permission Issues with Instruments
```bash
# Add user to dialout group for USB devices
sudo usermod -a -G dialout $USER
# Log out and back in for changes to take effect
```

### GPIB Interface Issues
```bash
# Install additional GPIB drivers if needed
sudo apt install gpib-modules-source
```

### Missing VISA Drivers
```bash
# Alternative VISA installation
sudo apt install python3-pyvisa-py  # Pure Python VISA implementation
```

### Build Errors
```bash
# Update Rust to latest version
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## For Developers

See the main README.md for development setup and contribution guidelines.