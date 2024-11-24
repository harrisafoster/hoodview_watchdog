# HoodviewWatchdog

## Goal

Create an executable Hoodview Campground scraper. This scraper's purpose is to check availability of campsites at their [website](https://www.recreation.gov/camping/campgrounds/232867) and report on results to the recipient's email address. 

## Local Dev

### Base Requirements

- Rust >= 1.82.0
- C/C++ build tools for your OS
- Docker >= 27.3.1
- Cross (cargo) >= 0.2.5
- Chromedriver >= 131.0.6778.85 (chromedriver must be on PATH)
- Terminal access (ideally Bash)

### Installation
##### Clone the source directly from Github:
```sh
$ git clone https://github.com/harrisafoster/hoodview_watchdog.git
$ cd hoodview_watchdog
```
##### Build for local (host) machine:
```sh
$ cd hoodview_watchdog
$ cargo build --release
```
##### Build for target (Windows) machine:
```sh
$ cd hoodview_watchdog
$ cross build --release --target x86_64-pc-windows-gnu 
```

### Use
#### To run the bot on local (host) machine:
```sh
$ cd hoodview_watchdog/
$ ./rust_modules/target/release/hoodview_watchdog
```

#### To run the bot on target (Windows) machine:
```sh
$ hoodview_watchdog.exe
```

#### Logging
All logs are available at ./log/hoodview_watchdog.log

#### Environment
HoodviewWatchdog relies on environment variables for account information. These are located in a .env file in the root (hoodview_watchdog) directory. You can find a template at ./template.env, this file should be copied, modified, and renamed to .env

#### Compatibility

HoodviewWatchdog can be run on any computer that supports c++/c lang compilers if built on host.

HoodviewWatchdog can be cross compiled for any target from this [list](https://docs.rs/crate/cross/latest) via "cross" command assuming Docker is present on the host system.