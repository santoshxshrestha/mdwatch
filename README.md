# mdwatch (Markdown Watcher CLI)

A simple command-line tool to preview Markdown files in a web browser. It serves the rendered HTML version of a Markdown file over HTTP, allowing you to easily preview your Markdown content locally.

## Features

- Serve a Markdown file as HTML via a local web server
- Automatically open the preview in your default web browser
- Supports specifying the file path, server IP, and port via command-line arguments
- Warns when binding to `0.0.0.0` to expose the server to the network
- Sanitizes rendered HTML to prevent injection of unsafe content

## Installation

Build the project using Cargo:

```bash
cargo build --release
```

---

## Usage

Run the tool with the required and optional arguments:

```bash
mdwatch --file README.md [--ip 127.0.0.1] [--port 23000]

```

- `--file`: Path to the Markdown file to preview (required)

- `--ip`: IP address to bind the server to (default: 127.0.0.1)

- `--port`: Port number for the server (default: 3000)

---

> [!NOTE]
> If you bind to `0.0.0.0`, the tool will warn you because this exposes the server to you local network.
> When binding to `0.0.0.0`, the preview URL opened in your browser will use `localhost` as the hostname.

---

# Example

```bash
mdwatch --file notes.md --ip 0.0.0.0 --port 8080
```

This will serve the Markdown file accessible on all network interfaces and open the preview at
`http://localhost:8080` in your browser.

---

# Security

- The rendered HTML is sanitized to prevent injection of unsafe content.
- Use caution when binding to `0.0.0.0` or any public-facing IP.
- This tool is intended for local development and preview purpose only.

---

## License

The project is made available under the MIT license. See the `LICENSE` file for more information.
