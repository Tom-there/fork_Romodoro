Romo - A Lightweight Pomodoro Timer

Romo is a simple and lightweight Pomodoro timer application that runs in your terminal. It helps you stay productive by managing work and break sessions.
Features

    Start a timer directly from your terminal.
    Receive desktop notifications when the timer completes.
    Cancel a running timer at any time.
    Installs itself globally (user-specific) for convenience.

Installation
Windows

    Download the latest romo.exe from the Releases page.

    Run the executable once to install it globally:

.\romo.exe

This installs Romo to C:\Users\<username>\.local\bin and adds it to your system PATH.

Restart your terminal (Command Prompt, PowerShell, etc.).

Verify the installation:

    romo --help

Linux

    Download the latest romo binary from the Releases page.

    Move the binary to a directory in your PATH:

chmod +x romo
./romo

This installs Romo to ~/.local/bin and ensures it is added to your shell PATH.

Restart your terminal.

Verify the installation:

    romo --help

Usage
Start a Timer

To start a Pomodoro timer, specify the duration in minutes:

romo 25

This will set a 25-minute timer.
Cancel a Timer

To cancel a running timer:

romo --cancel

Help

View available commands and options:

romo --help

Example Scenarios

    Start a 25-Minute Timer

romo 25

Cancel the Timer

romo --cancel

Start a Short 10-Minute Timer

    romo 10

Uninstallation
Windows

    Delete the romo.exe file from C:\Users\<username>\.local\bin.
    Remove C:\Users\<username>\.local\bin from your system PATH (optional).

Linux

    Delete the romo binary from ~/.local/bin:

    rm ~/.local/bin/romo

    Remove any PATH modifications from ~/.bashrc or ~/.zshrc if manually added.

Contributing

Contributions are welcome! If you encounter any issues or have suggestions, feel free to submit a pull request or open an issue in the GitHub repository.
License

This project is licensed under the MIT License.
Acknowledgements

Romo uses the following crates:

    clap for command-line parsing
    native-dialog for desktop notifications
    directories for user directory management

