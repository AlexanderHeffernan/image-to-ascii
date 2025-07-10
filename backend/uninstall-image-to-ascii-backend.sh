#!/bin/bash
set -e

echo "Starting uninstallation of Rust image-to-ASCII backend..."

# Set install directory (same as in install script)
INSTALL_DIR="$HOME/image-to-ASCII-backend"
SERVICE_FILE="/etc/systemd/system/image-to-ascii-backend.service"

# Stop and disable the systemd service
if sudo systemctl is-active image-to-ascii-backend.service >/dev/null 2>&1; then
    echo "Stopping image-to-ascii-backend service..."
    sudo systemctl stop image-to-ascii-backend.service
fi

if sudo systemctl is-enabled image-to-ascii-backend.service >/dev/null 2>&1; then
    echo "Disabling image-to-ascii-backend service..."
    sudo systemctl disable image-to-ascii-backend.service
fi

# Remove the systemd service file
if [ -f "$SERVICE_FILE" ]; then
    echo "Removing systemd service file..."
    sudo rm -f "$SERVICE_FILE"
    sudo systemctl daemon-reload
fi

# Remove the installation directory and all its contents
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing installation directory: $INSTALL_DIR"
    rm -rf "$INSTALL_DIR"
fi

echo "Rust image-to-ASCII backend has been successfully uninstalled."
echo "Note: This script does not remove curl or openssl as they may be used by other applications."
