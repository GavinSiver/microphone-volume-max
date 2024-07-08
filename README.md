# Windows Microphone Volume Max

## Overview

**Windows Microphone Volume Max** is a command-line tool written in Rust that sets the microphone volume to 100% on a recurring basis. It finds the default microphone and sets its volume to 100% every specified number of seconds. By default, it repeats this action every 60 seconds. Additionally, it can set the volume for the Krisp microphone if it is detected.

## Features

- Automatically sets the default microphone volume to 100%.
- Detects and sets the volume for the Krisp microphone to 100% if present.
- Configurable time interval for volume checks.
- Graceful shutdown with Ctrl-C signal handling.

## Requirements

- Rust (latest stable version recommended)
- Windows operating system
- [Clap](https://crates.io/crates/clap) for command-line argument parsing
- [Windows crate](https://crates.io/crates/windows) for Windows API bindings
- [Ctrlc](https://crates.io/crates/ctrlc) for handling Ctrl-C signals

## Installation

1. Clone the repository:
    \`\`\`sh
    git clone https://github.com/yourusername/windows-microphone-volume-max.git
    cd windows-microphone-volume-max
    \`\`\`

2. Build the project:
    \`\`\`sh
    cargo build --release
    \`\`\`

3. Run the executable:
    \`\`\`sh
    ./target/release/windows-microphone-volume-max
    \`\`\`

## Usage

The program can be run with the default settings or customized using command-line arguments.

### Command-Line Arguments

- \`-t, --time <TIME>\`: Time in seconds to wait before checking the volume again (default: 60 seconds). The value must be between 1 and the maximum value for a 64-bit unsigned integer.

### Examples

Run the program with the default settings (checks every 60 seconds):
\`\`\`sh
./windows-microphone-volume-max
\`\`\`

Run the program with a custom interval of 30 seconds:
\`\`\`sh
./windows-microphone-volume-max --time 30
\`\`\`

## Contributing

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes.
4. Push the branch to your fork.
5. Create a pull request.

## License

See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Clap](https://crates.io/crates/clap) for command-line argument parsing
- [Windows crate](https://crates.io/crates/windows) for Windows API bindings
- [Ctrlc](https://crates.io/crates/ctrlc) for handling Ctrl-C signals

## Contact

For any questions or suggestions, please open an issue.
