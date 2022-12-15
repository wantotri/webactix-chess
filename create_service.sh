#!/bin/bash
# Script for deploying Chess-Rust App
#   - Building the App
#   - Check if service is already active
#   - If active, restart the service
#   - If not active:
#     - Create Service file
#     - Reload daemon
#     - Enable service at startup
#     - Run the service

SERVICE_NAME=webactix-chess
APP_NAME=webactix

# Build the App
echo "Building Chess-Rust App"
cargo build --release

# Check if service is active
IS_ACTIVE=$(sudo systemctl is-active $SERVICE_NAME)

if [ "$IS_ACTIVE" == "active" ]; then
    # retarting service
    echo "service is already running"
    echo "restarting service"
    sudo systemctl restart $SERVICE_NAME
    echo "service restarted"

else
    # Creating service file
    echo "Creating Service File"
    sudo cat > /etc/systemd/system/${SERVICE_NAME}.service << EOF
[Unit]
Description=Webactix App for Chess Rust
After=network.target

[Service]
User=${USER}
Group=www-data
WorkingDirectory=${PWD}
ExecStart=${PWD}/target/release/${APP_NAME}

[Install]
WantedBy=multi-user.target
EOF
    # restart daemon, enable and start service
    echo "Reloading daemon and enabling service"
    sudo systemctl daemon-reload 
    sudo systemctl enable ${SERVICE_NAME//'.service'/} # remove the extension
    sudo systemctl start ${SERVICE_NAME//'.service'/}
    echo "Service Started"
fi

exit 0
