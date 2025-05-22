#!/bin/bash

echo "Creating deployment package for Linux..."
echo

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Cargo not found! Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the release
echo "Building release executable..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "ERROR: Build failed!"
    exit 1
fi

# Create deployment directory
if [ -d "lab-control-deployment" ]; then
    echo "Removing existing deployment directory..."
    rm -rf "lab-control-deployment"
fi

mkdir -p "lab-control-deployment"
mkdir -p "lab-control-deployment/frontend"
mkdir -p "lab-control-deployment/frontend/icons"

# Copy executable
echo "Copying executable..."
cp "target/release/automating_experiments" "lab-control-deployment/"

# Make executable runnable
chmod +x "lab-control-deployment/automating_experiments"

# Copy frontend files
echo "Copying frontend files..."
cp "frontend/index.html" "lab-control-deployment/frontend/"
cp frontend/icons/* "lab-control-deployment/frontend/icons/" 2>/dev/null || true

# Create shell script runner
echo "Creating run script..."
cat > "lab-control-deployment/run_lab_control.sh" << 'EOF'
#!/bin/bash

echo "====================================="
echo " Laboratory Experiment Control System"
echo "====================================="
echo

# Check if the executable exists
if [ ! -f "automating_experiments" ]; then
    echo "ERROR: automating_experiments executable not found!"
    echo "Please make sure you've built the project with: cargo build --release"
    echo "And copy the executable from target/release/ to this directory"
    exit 1
fi

# Check if frontend directory exists
if [ ! -d "frontend" ]; then
    echo "ERROR: frontend directory not found!"
    echo "Please make sure the frontend folder is in the same directory as this script"
    exit 1
fi

# Create data directory if it doesn't exist
if [ ! -d "data" ]; then
    echo "Creating data directory..."
    mkdir -p data
fi

# Display important information
echo "Starting the experiment control server..."
echo
echo "IMPORTANT NOTES:"
echo "- Make sure all VISA drivers are installed (libvisa, python3-pyvisa, etc.)"
echo "- Connect your instruments before running experiments"
echo "- The web interface will be available at: http://localhost:3000"
echo
echo "Press Ctrl+C to stop the server when you're done"
echo

# Run the application
./automating_experiments

# If the application exits, show a message
echo
echo "Application has stopped."
echo "Check the messages above for any errors."
read -p "Press Enter to continue..."
EOF

# Make the run script executable
chmod +x "lab-control-deployment/run_lab_control.sh"

# Create README for Linux
cat > "lab-control-deployment/README.txt" << 'EOF'
Laboratory Experiment Control System - Linux Deployment Package
===============================================================

SYSTEM REQUIREMENTS:
- Linux (Ubuntu 20.04+, CentOS 8+, or equivalent)
- VISA drivers installed (see VISA SETUP below)
- Laboratory instruments connected via USB/GPIB

VISA SETUP (Required):
For Ubuntu/Debian:
  sudo apt update
  sudo apt install libvisa-dev python3-pyvisa

For CentOS/RHEL/Fedora:
  sudo yum install libvisa-devel python3-pyvisa
  # or: sudo dnf install libvisa-devel python3-pyvisa

For NI-VISA (if available):
  Download from National Instruments website

INSTALLATION:
1. Extract this folder to any location on your computer
2. Make sure VISA drivers are installed (see above)
3. Ensure all instruments are connected and powered on
4. Open terminal in this directory
5. Run: ./run_lab_control.sh

USAGE:
1. The script will start the server
2. Open your web browser and go to: http://localhost:3000
3. Click "Check Connection" for each instrument you want to use
4. Select an experiment type and configure parameters
5. Click "Run Experiment" to start

SUPPORTED EXPERIMENTS:
- Current Sweep (CLD1015 + OSA)
- Wavelength Check (N77 + Power Meter)  
- Wavelength Sweep (N77 + Power Meter)
- Wavelength Sweep with OSA (N77 + OSA)

RESULTS:
All experiment results are saved in the "data/" folder as CSV files.

TROUBLESHOOTING:
- If instruments don't connect, check VISA drivers and connections
- If permission denied on run script: chmod +x run_lab_control.sh
- If the web interface doesn't load, ensure no other application is using port 3000
- Press Ctrl+C in terminal to stop the server

INSTRUMENT ADDRESSES (configured in the software):
- CLD1015: USB::4883::32847::M01053290::0::INSTR
- N7714A: GPIB0::21::INSTR
- MPM210-H: GPIB0::16::INSTR
- HP-70952B: GPIB0::23::INSTR

For technical support, contact: [Your contact information]
EOF

echo
echo "✓ Linux deployment package created successfully!"
echo "✓ Location: lab-control-deployment/"
echo
echo "You can now:"
echo "  1. Test it by running: cd lab-control-deployment && ./run_lab_control.sh"
echo "  2. Create a tar.gz archive: tar -czf lab-control-system-linux.tar.gz lab-control-deployment/"
echo "  3. Copy to Linux lab computers"
echo
echo "Press Enter to continue..."
read