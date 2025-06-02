#!/bin/bash

echo "====================================="
echo " Laboratory Experiment Control System"
echo " Linux Setup and Build Script"
echo "====================================="
echo

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo "Please don't run this script as root/sudo"
    echo "The script will ask for sudo when needed"
    exit 1
fi

# Install system dependencies
echo "Step 1: Installing system dependencies..."
echo "This requires sudo privileges:"

# Detect distribution
if command -v apt-get &> /dev/null; then
    # Ubuntu/Debian
    sudo apt-get update
    sudo apt-get install -y libvisa-dev python3-pyvisa libusb-1.0-0-dev pkg-config libssl-dev curl build-essential
elif command -v yum &> /dev/null; then
    # CentOS/RHEL
    sudo yum groupinstall -y "Development Tools"
    sudo yum install -y libvisa-devel python3-pyvisa libusb-devel pkgconfig openssl-devel curl
elif command -v dnf &> /dev/null; then
    # Fedora
    sudo dnf groupinstall -y "Development Tools" 
    sudo dnf install -y libvisa-devel python3-pyvisa libusb-devel pkgconfig openssl-devel curl
else
    echo "Unsupported distribution. Please install VISA development libraries manually."
    echo "Required packages: libvisa-dev, python3-pyvisa, libusb-dev, pkg-config, openssl-dev"
    read -p "Continue anyway? (y/n): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo "✓ System dependencies installed"
echo

# Install Rust
echo "Step 2: Checking Rust installation..."
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    echo "✓ Rust installed"
else
    echo "✓ Rust already installed"
fi
echo

# Add user to dialout group for USB device access
echo "Step 3: Setting up USB device permissions..."
if ! groups $USER | grep -q dialout; then
    echo "Adding user to dialout group for USB device access..."
    sudo usermod -a -G dialout $USER
    echo "⚠️  You'll need to log out and back in for USB permissions to take effect"
else
    echo "✓ User already in dialout group"
fi
echo

# Create data directory
echo "Step 4: Creating data directory..."
mkdir -p data
echo "✓ Data directory created"
echo

# Build the application
echo "Step 5: Building the application..."
echo "This may take several minutes on first build..."
echo

if cargo build --release; then
    echo
    echo "Build successful!"
    echo
    echo "====================================="
    echo " Setup Complete!"
    echo "====================================="
    echo
    echo "To start the Lab Control System:"
    echo "  ./target/release/automating_experiments"
    echo
    echo "Or use the quick start script:"
    echo "  ./run_lab_control.sh"
    echo
    echo "Web interface will be available at:"
    echo "  http://localhost:3000"
    echo
    echo "Note: If you had USB permission changes, please log out"
    echo "and back in before connecting instruments."
    echo
else
    echo
    echo "Build failed!"
    echo
    echo "Common solutions:"
    echo "1. Make sure all dependencies are installed"
    echo "2. Update Rust: rustup update"
    echo "3. Clean build: cargo clean && cargo build --release"
    echo
    echo "For help, check the LINUX_README.md file"
    exit 1
fi