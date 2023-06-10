# Tauri + Svelte + Typescript

A lot of this project is code from Spyglass, as it was the perfect base to start
from handling most of the things I was after.

## Screenshots

<img width="765" alt="image" src="https://github.com/samjwillis97/searcher-rs/assets/37866085/3c4cdc2d-322d-497d-8330-aa1241613d63">

<img width="765" alt="image" src="https://github.com/samjwillis97/searcher-rs/assets/37866085/ce2ffcad-8b63-4b97-8283-20b14a422f02">


## Goals

- Parses the config file - error popup if fails
- Reloads config on change

## TODO

- Taskbar Icon
- `n` and `p` are not enterable in the search
- Work out where the logs go - for windows especially
- Modify the pipeline to run less often
- On Lose focus hide window
- Wildcard search (ALL?)
- Allow searching across multiple fields, just don't do the highlighting
- Make sure display name is working

## Completed

- Fix the config file path
- initial search will search the configs
- Screen position of the search bar in config
- Application starts (on startup) to system tray
- A global hotkey (similar to PrtScrn) opens up the search window
- it will also accept hotkeys as defined in the config
- Only display the selected fields, not necessarily the searched fields
- Escape back to the initial search - maybe drive by config
- Escape used as a back button rather than escaping the programing
- Pressing escape/shortcut in the search window will close it and clear search
  - effectively resets the search


## Config

- `config.yaml` contains main config like
  - `entries_to_show`
  - `fzf_algorithm`
  - `simliarty`
  - `shortcut`
- `services/` contains a set of search configurations for the program
- `services/<unique_service_name>.yaml` service configuration
  - `name`
  - etc.

## Run

`pnpm tauri dev`
