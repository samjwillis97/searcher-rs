# Tauri + Svelte + Typescript

A lot of this project is code from Spyglass, as it was the perfect base to start from handling most of the things I was after.

## Goals

- Application starts (on startup) to system tray
- Parses the config file - error popup if fails
- Reloads config on change
- A global hotkey (similar to PrtScrn) opens up the search window
- initial search will search the configs
- it will also accept hotkeys as defined in the config
- Pressing escape/shortcut in the search window will close it and clear search
    - effectively resets the search


## TODO

- Show all servies in list at boot
- Show hotkeys assigned on the right hand side
- Similar to spyglass show the service being used in the search bar somehow
- Screen position of the search bar in config
- Escape back to the initial search - maybe drive by config
- Escape used as a back button rather than escaping the programing

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

