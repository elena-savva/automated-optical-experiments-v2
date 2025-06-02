#!/bin/bash

echo "====================================="
echo " Laboratory Experiment Control System"
echo "====================================="
echo

# Check if executable exists
if [ ! -f "target/release/automating_experiments" ]; then
    echo "Application not built yet. Please run setup first:"
    echo "  ./linux_setup.sh"
    echo
    exit 1
fi

# Create data directory
mkdir -p data

# Display startup info
echo "Starting Lab Control System..."
echo
echo "Web interface: http://localhost:3000"
echo "Connect your instruments before running experiments"
echo "Press Ctrl+C to stop the server"
echo

# Check for VISA libraries
if ! ldconfig -p | grep -q libvisa; then
    echo "⚠️  WARNING: VISA libraries not detected"
    echo "   Install with: sudo apt install libvisa-dev python3-pyvisa"
    echo
fi

# Check USB permissions
if ! groups $USER | grep -q dialout; then
    echo "⚠️  WARNING: User not in dialout group"
    echo "   USB instruments may not be accessible"
    echo "   Run: sudo usermod -a -G dialout $USER"
    echo "   Then log out and back in"
    echo
fi

# Start the application
echo "Starting server..."
echo
./target/release/automating_experiments