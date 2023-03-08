# Python in Rust sample project
This is a mini project from IDS721. The main goal of this project is trying to use python in rust.

## Usage
Type in ```cargo run``` to start the program.

## Potential issues
1. When I try this project local on M1 macos, it is possible to show a cc error. For this issue, you should adjust the config file which is located in ```~/.cargo/config```. 
2. It is possible to show an error like python mod not found. One potential solution is to use ```sudo apt install python3-dev``` to install required environment.
3. For local, you might need to install pyenv, then install the required environment.
## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
