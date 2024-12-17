# Romo - A Lightweight Pomodoro Timer

Romo is a simple and lightweight Pomodoro timer application that you start in your terminal, but runs in the background. When the timer runs out, it will alert you with a dialog box telling you to take a break.

## Installation
### Windows
1. Download the latest romo.exe from the [Releases](https://github.com/dj-blume9/Romodoro/releases) page.
2. Open the romo.exe to install once, globally.

### Linux
1. Download the latest romo binary from the [Releases](https://github.com/dj-blume9/Romodoro/releases) page.
2. Run the following commands to install it globally:
    ``
   tar -xvzf romo-linux.tar.gz
   ./romo
   ``
3. Restart your terminal


## Usage
1. Start a timer by calling "romo" followed by the number of minutes for your timer.
    ```
    //Start a 10 minute timer
    romo 10
    ```
2. Cancel a running timer by calling
   ```
   romo --cancel
   ```
