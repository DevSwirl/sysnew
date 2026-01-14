ArchBoost (v1)

ArchBoost is a Rust-based single-run bootstrap tool for Arch-based Linux systems (Arch Linux, EndeavourOS, Garuda, Hyprland). It installs essential tools, enables safe performance optimizations, and applies beginner-friendly privacy defaults on a fresh system.

This version is designed for fresh installs only. It avoids unsafe or destructive operations.

Features
Performance and Stability

Enables SSD TRIM

Installs earlyoom to prevent system freezes

Installs PipeWire audio stack

Installs performance monitoring tools (btop, htop)

Updates the system

Privacy and Security (Safe Defaults)

Enables UFW firewall with default deny incoming and allow outgoing

Installs Signal Desktop

Uses only official pacman packages

Safe for first startup, no network lockouts

No DNS or IPv6 changes

Developer and Power-User Tools

Git, base-devel, linux headers

Rust toolchain via rustup

Neovim

CLI utilities: bat, exa, ripgrep, fzf, zoxide

Fastfetch system info

Desktop and Productivity Apps

LibreOffice

Thunderbird

GIMP

Obsidian

PipeWire + WirePlumber

Bluetooth stack (BlueZ)

Supported Systems:

Arch Linux

EndeavourOS

Garuda

Hyprland-based installs

GNOME / KDE / X11 / Wayland

Requirements:

Working desktop environment or window manager

systemd

pacman package manager

User with sudo access

Usage
git clone https://github.com/DevSwirl/ArchBoostV1.git
cd 
cargo build --release
./target/release/archboost


You will be prompted for your sudo password.

Limitations

This version does not:

Modify kernel or sysctl beyond safe defaults

Replace DNS or enforce DoH

Configure nftables/iptables

Enable Tor routing or VPN kill switches

Remount filesystems or change permissions

Apply desktop-specific tweaks

These features may be added in future versions as optional modules.

Roadmap

v2: Safe kernel and sysctl hardening (opt-in)

v3: DNS privacy and leak protection

v4: VPN/Tor profiles

v5: Idempotency and rollback support

Profile system: balanced, privacy, paranoid

Contributing

Pull requests are welcome. When contributing:

Keep changes opt-in and safe for first startup

Use official Arch repositories

Avoid desktop-specific breakage

License

MIT
