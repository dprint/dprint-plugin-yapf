# Script for quickly creating the plugin for testing purposes on Windows
# To run:
# 1. Comment out osx and linux `getPlatformObject` and change the reference line to point to zip output `./${zipFileName}` in scripts/createPluginFile.js
# 2. Run `./scripts/createForTestingOnWindows.ps1`
# 3. Update dprint.json to point at ./yapf.exe-plugin then update checksum as shown when initially run.

cd executable
cargo build --release --locked --all-targets --verbose
cp ../main.py target/release/main.py
cd target/release
Compress-Archive -CompressionLevel Optimal -Force -Path dprint-plugin-yapf.exe, main.py -DestinationPath ../../../dprint-plugin-yapf-x86_64-pc-windows-msvc.zip
cd ../../../
node scripts/createPluginFile.js
