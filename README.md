# bevy_demo_2048
Game 2048 by rust-bevy

- [ ] Add the below `.cargo/config.toml` file for the script `launch-devices.sh` to work

```toml
# Metadata used when generating an iOS APP, for example.

# To find a certificate in the keychain
# certtool y | grep Steve Works 

# to find a provisioning file in the provisioning directory
# ls -l ~/Library/MobileDevice/Provisioning\ Profiles

[package.metadata.signing]
signature = "Apple Development: Steve Works (H3JYSD4DDT)"
provisioning_file = "SJ_provisioning_profile.mobileprovision"
```
# References

- [ ] [Native iOS Touch Events w/ Rust](https://dev.to/wadecodez/bevy-native-ios-touch-events-49p3)
