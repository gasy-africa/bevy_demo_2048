#/usr/bin/env bash

APP_NAME="$(cat Cargo.toml | dasel -r toml '.package.name')"
BUNDLE_ID="$(cat Cargo.toml | dasel -r toml '.package.metadata.bundle.identifier')"
ARCH=$(uname -m)

# xcrun simctl list to get the DEVICE ID
# xcrun simctl boot "iPhone 14 Pro"  
DEVICE_ID=81989D13-9AA7-4CCC-9317-6FE729E059D4
xcrun simctl boot ${DEVICE_ID}
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app 

# Compile in a specific platform
cargo bundle --target ${ARCH}-apple-ios # x86_64-apple-ios-sim

# Install the App onto the booted device
xcrun simctl install booted "target/${ARCH}-apple-ios/debug/bundle/ios/$APP_NAME.app"
# Copy the assets onto the booted device
cp -r assets "$(xcrun simctl get_app_container booted ${BUNDLE_ID})"

# Print logs to the console
xcrun simctl launch --console booted "${BUNDLE_ID}"


