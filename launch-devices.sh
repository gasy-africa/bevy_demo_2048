#/usr/bin/env bash

# DEVICE_ID=$(xcrun xctrace list devices | grep Ankorahotra | awk '{print $3}' | tr -d '()' )
DEVICE_ID=${1}

# usage: bash launch-devices.sh ${DEVICE_ID}

ARCH=aarch64
APP_NAME="$(cat Cargo.toml | dasel -r toml '.package.name')"

BUNDLE_ID="$(cat Cargo.toml | dasel -r toml '.package.metadata.bundle.identifier')"

# Signing
SIGNATURE="$(cat Signing.toml | dasel -r toml '.package.metadata.signing.signature')"
PROVISIONING_FILE="$(cat Signing.toml | dasel -r toml '.package.metadata.signing.provisioning_file')"

# Add a specific platform
rustup target add ${ARCH}-apple-ios

# Compile in a specific platform
cargo bundle --target ${ARCH}-apple-ios 

# Extract the profile payload
# https://developer.apple.com/documentation/technotes/tn3125-inside-code-signing-provisioning-profiles
# CMS https://developer.apple.com/documentation/security/cryptographic_message_syntax_services 
security cms -D -i ~/Library/MobileDevice/Provisioning\ Profiles/${PROVISIONING_FILE} -o Payload.plist

# Extract the Entitlements with property list util tool
plutil -extract Entitlements xml1 -o - Payload.plist > Entitlements.plist 

# Code Sign the apps
codesign --force --verbose --sign "${SIGNATURE}" --entitlements Entitlements.plist "target/${ARCH}-apple-ios/debug/bundle/ios/$APP_NAME.app"
# Error: The specified item could not be found in the keychain.
# target/aarch64-apple-ios/debug/bundle/ios/ios_touch_events.app: signed bundle with Mach-O thin (arm64) [com.valiha.fanorona]

# Check is code is signed
codesign -dvvvv "target/${ARCH}-apple-ios/debug/bundle/ios/$APP_NAME.app"
# target/aarch64-apple-ios/debug/bundle/ios/ios_touch_events.app/ios_touch_events: code object is not signed at all

# Remove the poperty lists used to code sign
rm Payload.plist Entitlements.plist


# Copy assets in bundle
cp -r assets "target/${ARCH}-apple-ios/debug/bundle/ios/$APP_NAME.app"

# Install the App onto the booted device
# DEBUG="--debug"
ios-deploy ${DEBUG} \
           --id ${DEVICE_ID} \
           --bundle_id ${BUNDLE_ID}  \
           --bundle "target/${ARCH}-apple-ios/debug/bundle/ios/$APP_NAME.app"

# Upload a file to your app's Documents folder
# ios-deploy --bundle_id ${BUNDLE_ID} --upload assets --to assets
# ios-deploy --id ${DEVICE_ID} --bundle_id ${BUNDLE_ID} --mkdir assets
# ios-deploy --id ${DEVICE_ID} --bundle_id ${BUNDLE_ID} --mkdir assets/sounds
# ios-deploy --id ${DEVICE_ID} --bundle_id ${BUNDLE_ID} --upload "assets/sounds/Windless Slopes.ogg" --to "assets/sounds/Windless Slopes.ogg"


# Error: A valid provisioning profile for this executable was not found. 

