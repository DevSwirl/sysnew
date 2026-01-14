# ArchBoost

Simple Rust CLI tool for bootstrapping a fresh Arch-based Linux system with essential packages, performance optimizations, and safe privacy defaults.

Install the binary in `/usr/local/bin` and run `archboost` from anywhere.

## Requirements

- Arch Linux or Arch-based distro (EndeavourOS, Garuda, Hyprland, etc.)
- pacman
- Rust (cargo, rustc)
- sudo

## Installation

Install Rust if needed:

sudo pacman -S rustup
rustup default stable

Clone the repository:

cd Desktop

git clone https://github.com/DevSwirl/ArchBoostV1.git

cd ArchBoostV1

cd sysnew

cd src

Build the project:

rustc sysnew.rs

Transfer binary for system-wide use:

sudo mv /home/{YOUR_NAME}/Desktop/ArchBoostV1/sysnew/src/sysnew /usr/local/bin/

Usage

Run:

sysnew

You will be prompted for your sudo password. The program will install essential packages, enable services like earlyoom and fstrim.timer, and configure safe defaults for a fresh system.
Development

Run without installing:

cargo run

License

MIT
