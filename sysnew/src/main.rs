use std::io;
use std::process::Command;
use std::{thread, time::Duration};

fn main() {



    println!("
┏┓   •  ┓  ┏┓   ┳┳   ┓     
┗┓┓┏┏┓┏┓┃  ┗┓
┗┛┗┻┛┗┛ ┗  
      ┛        ");


    

    println!("Press Enter to begin...");


    let mut enter = String::new();
    io::stdin()
    .read_line(&mut enter)
    .expect("ERROR");

    thread::sleep(Duration::from_millis(800));

    println!("PRIVACY/SPEED ARCH INCOMINGGG!!!");

    thread::sleep(Duration::from_millis(3000));

commands()
}


fn commands() {



    Command::new("sudo")
    .arg("-v")
    .status()
    .expect("ERROR");



    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "reflector"])
    .status()
    .expect("ERROR");



    Command::new("sudo")
    .args(["pacman", "-Syu"])
    .status()
    .expect("ERROR");



    Command::new("sudo")
    .args(["pacman", "-S", "btop", "libreoffice-fresh", "thunderbird", "gimp", "obsidian", "signal-desktop"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["pacman", "-S", "base-devel", "linux-headers"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "git", "curl", "wget", "rsync", "unzip", "zip", "htop", "neovim", "fastfetch"])
    .status()
    .expect("ERROR");



    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "util-linux"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["systemctl", "enable", "--now", "fstrim.timer"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "earlyoom"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["systemctl", "enable", "--now", "earlyoom"])
    .status()
    .expect("ERROR");



    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "bluez", "bluez-utils"])
    .status()
    .expect("ERROR");

   

    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "pipewire", "pipewire-alsa", "pipewire-pulse", "pipewire-jack", "wireplumber"])
    .status()
    .expect("ERROR");



    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "rustup"])
    .status()
    .expect("ERROR");


    Command::new("rustup")
    .args(["default", "stable"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "git", "base-devel"])
    .status()
    .expect("ERROR");



    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "bat", "exa", "fd", "ripgrep", "fzf", "zoxide"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "ufw"])
    .status()
    .expect("ERROR");

    Command::new("sudo")
    .args(["systemctl", "enable", "--now", "ufw"])
    .status()
    .expect("ERROR");

    Command::new("sudo")
    .args(["ufw", "default", "deny", "incoming"])
    .status()
    .expect("ERROR");

    Command::new("sudo")
    .args(["ufw", "default", "allow", "outgoing"])
    .status()
    .expect("ERROR");

    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "tlp"])
    .status()
    .expect("ERROR");

    Command::new("sudo")
    .args(["systemctl", "enable", "--now", "tlp"])
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .args(["pacman", "-S", "--needed", "spice-vdagent"])
    .status()
    .expect("ERROR");





}