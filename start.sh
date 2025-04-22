#!/bin/bash

# Set environment variables
export LIVEKIT_API_KEY="devkey"
export LIVEKIT_API_SECRET="secret"
export LIVEKIT_SERVER_ADDRESS="localhost:7880"
# export FISHNET_SERVER_ADDRESS="localhost:7777"
export FISHNET_SERVER_ADDRESS="[::1]:7777"
export TOKENSERVER_PORT="3030"

# Start the application
cargo run
