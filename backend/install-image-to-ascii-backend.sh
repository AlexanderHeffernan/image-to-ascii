#!/bin/bash
set -e

echo "Starting installation of Rust image-to-ASCII backend..."

# Check if required tools are installed
command -v curl >/dev/null 2>&1 || { echo "curl is required. Please install it (e.g., 'sudo apt install curl')."; exit 1; }
command -v openssl >/dev/null 2>&1 || { echo "OpenSSL is required. Please install it (e.g., 'sudo apt install openssl')."; exit 1; }

# Set install directory
INSTALL_DIR="$HOME/image-to-ASCII-backend"
mkdir -p "$INSTALL_DIR"
cd "$INSTALL_DIR"

# Download the pre-built binary
echo "Downloading Rust image-to-ASCII backend binary..."
curl -sSL -o image-to-ASCII https://raw.githubusercontent.com/AlexanderHeffernan/image-to-ASCII/main/backend/bin/image-to-ASCII
chmod +x image-to-ASCII

# Generate TLS certificates if not present
if [ ! -f certs/cert.pem ] || [ ! -f certs/key.pem ]; then
    echo "Generating TLS certificates..."
    mkdir -p certs
    openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem -days 365 -nodes -subj "/CN=localhost"
fi

echo "TLS certificates are ready."

# Create systemd service file
SERVICE_FILE="/etc/systemd/system/image-to-ascii-backend.service"
echo "Configuring systemd service..."
sudo bash -c "cat > $SERVICE_FILE" <<EOL
[Unit]
Description=Rust image-to-ASCII Backend
After=network.target

[Service]
ExecStart=$INSTALL_DIR/image-to-ASCII
WorkingDirectory=$INSTALL_DIR
Restart=always
User=$(whoami)

[Install]
WantedBy=multi-user.target
EOL

# Reload and enable the service
sudo systemctl daemon-reload
sudo systemctl enable image-to-ascii-backend.service
sudo systemctl start image-to-ascii-backend.service

# Verify the service is running
if sudo systemctl is-active image-to-ascii-backend.service >/dev/null; then
    echo "Rust image-to-ASCII backend installed and running."
else
    echo "Failed to start Rust image-to-ASCII backend. Check logs with 'journalctl -u image-to-ascii-backend.service'."
    exit 1
fi