# my_status

Custom implementation of i3status, written in Rust.

## Features

The program consists of different modules, each of which can add a status block
displayed as text separated with bars. Here's an overview of the modules and
their functionality:

- battery: Displays the current battery percentage and charge state.
- datetime: Displays both the date and the time.
- bluetooth: Displays all connected bluetooth devices.
    - Name of the device
    - Reads certain BLE values:
        - Heart rate (in bpm)
        - Battery (in percentage)
- network: Displays the state of wpa_supplicant
    - Connection status is displayed with colors:
        - Green: connected
        - Red: disconnected/wpa_supplicant not running
        - Orange: everything else (e.g. connecting)
    - SSID of the connected network
    - Current IP adress
- playing: Displays the currently playing song using D-Bus and playerctl
    - Playing/Paused/Stopped status
    - Song title and (if available) artists

## Legacy

This project has gone through many stages, here's an overview:

- [Version
  1](https://github.com/Robbe7730/dotfiles/blob/master/scripts/myStatus):
  Written in plain bash
- [Version
  2](https://github.com/Robbe7730/dotfiles/blob/master/scripts/myStatus_python):
  Written in Python
- [Version 3](https://github.com/Robbe7730/my_status/tree/v3.0.0): Written in
  Rust, but without any Rust experience
- Version 4: The one you see in this repository
