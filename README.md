# Waybat

Waybat is a lightweight battery tray indicator written in Rust.

## Table of Contents

- [Description](#description)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [ToDo](#todo)

## Description

Waybat is a Rust-based application that monitors your battery level in real time and displays a dynamic icon in the system tray. The icon changes according to the battery charge percentage and charging status. This project uses the following crates:
- **tray-item** (with the `libappindicator` feature)
- **gtk**
- **battery**

## Features

- **Real-time monitoring:** Periodically reads the battery level.
- **Dynamic tray icon:** Updates the icon based on battery percentage and charging status.
- **Cross-platform support:** Works on Linux (using libappindicator) and can be adapted for other platforms.
- **Easy configuration:** Customize update intervals and icon thresholds through configuration files.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (recommended version: stable)
- System dependencies (for Linux):
  - `libgtk-3-dev`
  - `libappindicator3-dev`

### Clone the Repository

```bash
git clone https://github.com/mrlpm/waybat.git
cd waybat
```

## Usage

To run the application in development mode, execute:

```bash
cargo run --release
```
The tray icon will appear in your system tray and update periodically according to the battery status.

## ToDo

- [ ] Configuration file
- [ ] Notifications for battery changes
- [ ] Log to a file
