# Projext

This fun new project to show off my rust programming skills by using cool font, text to speech and other things. 

Requirements:
- MacOS ( has only been tested version MacOS 13)
- Xcode with a device simulator (ex iPhone 14 pro simulator)
- requires neofetch command installed (can be installed via homebrew package manager)
- requires Android studio with a device simulator (ex pixel 6 pro)
- requires cmatrix command installed (can be installed via homebrew package manager)
- requires rust installed
- requires speedest command installed (can be installed via homebrew pakage manager)

Note to run this you will need to edit and change the file paths in files as currently this program only works in the testing environment and is not ready for production usage by others. Paths that you need to change are variable script path in main.rs, change the path in scripts directory as well as the device command in android.sh, change the Xcode simulator path to the device inside the main.rs.

To run this program:
```
cargo run 
```
To Build this program for production: (if you don't want to build for production remove --release tag): 
```
cargo build --release
```
