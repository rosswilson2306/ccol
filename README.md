# Ccol - Command Collection

## Description

Tui application for easily accessing a personal store of commands for different CLI tools.

`ccol` aims to provide a simple interface for CLI commands that can either be tedius to type out or difficult to remember.

Born from a desire to provide a better way of storing `curl` commands piped to `jq` than storing them in a `yaml` file, which
became a way of storing other commands that can be long to type out or have flags or syntax that can be difficult to remember.
`ccol` reads from a config file meaning the user does not have to manually `cd` to a directory / file on their system to copy 
commands they have stored.

## Getting Started

Clone this repository, build with:
```bash
cargo build --release
```
move the generated binary somewhere within you `$PATH`, eg:
```bash
mv ./target/release/ccol /local/bin
```

## Usage

```bash
ccol
```

### Configuration

`ccol` reads from a JSON config file within your system's XDG config directory (`$HOME/.config/ccol/ccol.json` on Linux).
This file will be generated on startup but currently needs to be manually configured in order to create your command collection.

The `ccol.json` file must follow these rules:
- the root of the file must be a JSON object `{}`
- the key: value pairs can have a value that is either a string or a JSON object, representing a command or a sub collection respectively

Example `ccol.json`
```json
{
    "system": {
        "search-system-fonts": "fc-list | fzf",
        "set-shell": "chsh -s $(which zsh)",
    }
}
```

## Environment Variables

`CCOL_CONFIG_PATH` overwrites your system's default XDG config directory.

```bash
CCOL_CONFIG_PATH=/Users/my-user/.config
```

## TODO

- install script
- editing / adding / deleting commands within TUI
