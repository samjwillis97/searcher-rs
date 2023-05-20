# Tauri + Svelte + Typescript

A lot of this project is code from Spyglass, as it was the perfect base to start
from handling most of the things I was after.

## Goals

- Parses the config file - error popup if fails
- Reloads config on change
- initial search will search the configs

## TODO

- Wildcard search (ALL?)
- Screen position of the search bar in config
- Allow searching across multiple fields, just don't do the highlighting
- Make sure display name is working

## Completed

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
