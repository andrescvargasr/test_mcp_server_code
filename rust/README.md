# LED Control Client (Rust)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MCP](https://img.shields.io/badge/Protocol-MCP-red)](https://modelcontextprotocol.io/)

Rust implementation of a command-line client to control an LED device using the **Model Context Protocol (MCP)** over HTTP/JSON-RPC 2.0.

## Features

- Communicates with MCP server over HTTP JSON-RPC 2.0 (`reqwest`, `serde`, `serde_json`).
- Handles MCP Session Initialization (`initialize`) and notification flow (`notifications/initialized`).
- Supports querying available server tools via `tools/list` using the `list` action or `--list-tools` (`-l`, `--list`) flag.
- Supports custom mDNS hostnames (`--mdns`) and HTTP ports (`--port`) as flags or positional arguments.
- Executes `led_control` tool calls with dynamic JSON parameter building.
- Supports actions: `on`, `off`, `toggle`, `red`, `green`, `blue`, `list`.
- Supports RGB channels (`--r`, `--g`, `--b`), CSS color strings (`--color`), rainbow animation (`--rainbow`), and LED index selection (`--index`).

## Build & Prerequisites

Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).

### Build

```bash
cargo build --release
```

The compiled binary will be available at `./target/release/control_led`.

## Usage & Command Syntax

### Running via Cargo

```bash
cargo run -- [action] [mdns] [port] [options]
```

### Options & Flags

| Flag / Option | Default | Description |
| --- | --- | --- |
| `--action <action>` | — | LED action: `on`, `off`, `toggle`, `red`, `green`, `blue`, `list` |
| `--mdns <hostname>` | `mcp-led` | Hostname prefix for target device (`http://{mdns}.local:{port}/mcp`) |
| `--port <port>` | `8080` | Target server HTTP port |
| `--list-tools` / `-l` / `--list` | `false` | Fetch and display available tools from the server via `tools/list` |
| `--r <0-255>` | — | Red channel intensity |
| `--g <0-255>` | — | Green channel intensity |
| `--b <0-255>` | — | Blue channel intensity |
| `--color <string>` | — | Color string (e.g., `'rgb(255,0,0)'`) |
| `--rainbow` | `false` | Enable rainbow animation effect |
| `--index <val>` | — | LED position (`0-...`) or `256` for all |
| `-h`, `--help` | — | Display usage instructions |

> **Note**: Positional arguments are also supported: `cargo run -- [action] [mdns] [port]`

## Usage Examples

### Listing Available Tools (`tools/list`)

```bash
# List tools using the 'list' action
cargo run -- list

# List tools using the optional flag
cargo run -- --list-tools

# List tools on a custom server
cargo run -- --list-tools my-custom-led 9090
```

### Basic Commands (Default Host: `http://mcp-led.local:8080/mcp`)

```bash
# Turn LED ON / OFF / Toggle
cargo run -- on
cargo run -- off
cargo run -- toggle

# Set LED color to Red, Green, or Blue
cargo run -- red
cargo run -- green
cargo run -- blue
```

### Custom Color & Animation Controls

```bash
# Custom RGB values
cargo run -- --r 255 --g 128 --b 0

# Custom CSS Color string
cargo run -- --color "rgb(255,0,0)"

# Enable Rainbow effect
cargo run -- --rainbow

# Target Specific LED Index (e.g., LED position 0, or 256 for all)
cargo run -- --r 255 --g 0 --b 0 --index 0
cargo run -- toggle --index 256
```

### Custom Server Hostname & Port

```bash
# Positional hostname & port
cargo run -- toggle my-custom-led 9090

# Explicit connection flags
cargo run -- --rainbow --mdns my-custom-led --port 9090
```

### Running Built Binary

```bash
./target/release/control_led on --mdns my-custom-led --port 9090
./target/release/control_led --list-tools
```
