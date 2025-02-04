# Ccol - Command Collection

## Description

`ccol` is a TUI application desgined for easy access to a personal store of CLI comands across various tools.

It provides a simple interface for managing commands that are either tedious to type or difficult to remember. 
Initially inspired by the need for a better way to store `curl` commands piped to `jq` - as opposed to keeping them
in a `yaml` file - `ccol` evolved into a general-purpose solution for storing an retrieving complex CLI commands.

By reading from a configuration file, `ccol` eliminates the need to manually navigate directories or open files to 
copy stored commands, making execution more seamless and efficient.

## Getting Started

### Prerequesites

- [rustup](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Build from source

Clone this repository then build with:
```bash
cargo build --release
```
Move or copy the generated binary to a location available within you `$PATH`, eg:
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
    },
    "curl": {
        "my-api": {
            "find": "curl 'https://mp-long-url.com?page=1&page_size=12&sort=price-asc"
        }
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
