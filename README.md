# mdwatch (Markdown Watcher CLI)

<a href="https://repology.org/project/mdwatch/versions">
    <img src="https://repology.org/badge/vertical-allrepos/mdwatch.svg" alt="Packaging status" align="right">
</a>

A simple command-line tool to preview Markdown files in a web browser. It serves the rendered HTML version of a Markdown file over HTTP, allowing you to easily preview your Markdown content locally.

## Features

- Serve a Markdown file as HTML via a local web server
- Automatically open the preview in your default web browser
- Auto reload on save
- Supports specifying the file path, server IP, and port via command-line arguments
- Warns when binding to `0.0.0.0` to expose the server to the network
- Sanitizes rendered HTML to prevent injection of unsafe content

# Installation

You have three options: via Cargo, via prebuilt script, or manual install.

### 1. Easiest: Install via Cargo (Recommended)

If you have Rust installed, you can install directly from [crates.io](https://crates.io):

```bash
cargo install mdwatch
```

This is the most "Rusty" and portable way.  
It automatically downloads, compiles, and installs the latest version to your `$HOME/.cargo/bin`.

> If you want even faster installs with prebuilt binaries, check out [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall mdwatch
```

---

### 2. Quick Install via Script

**Alternative:** Installs the latest release binary to your system PATH.

```bash
curl -sSfL https://raw.githubusercontent.com/santoshxshrestha/mdwatch/main/scripts/install.sh | bash
```

- This script will:
  1. Build `mdwatch` in release mode (if Rust is present).
  2. Copy the binary to `/usr/local/bin`.
  3. Make it executable.

> **Tip:** You may need to enter your password for `sudo` privileges.

---

### 3. Manual Build & Install

If you prefer full control or want to customize the build:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/santoshxshrestha/mdwatch.git
   cd mdwatch
   ```

2. **Build the Release Binary:**

   ```bash
   cargo build --release
   ```

   This places the binary at `target/release/mdwatch`.

3. **Copy to a PATH directory (e.g., `/usr/local/bin`):**

   ```bash
   sudo cp target/release/mdwatch /usr/local/bin/mdwatch
   ```

4. **(Optional) Ensure executable permission:**

   ```bash
   sudo chmod +x /usr/local/bin/mdwatch
   ```

5. **Run from anywhere:**

   ```bash
   mdwatch
   ```

---

## Using with Nix (Flakes)

If you have [Nix](https://nixos.org/download.html) (version 2.4 or later) with flakes enabled, you can use `nix run` to run `mdwatch` without installing Rust or any dependencies globally. You can also use `nix develop` to get a full development environment for hacking on the project.

### Run mdwatch instantly (no install needed)

#### From the current directory

```bash
nix run
```

This will build and run the latest version of `mdwatch` from the flake in the current directory. You can pass arguments as usual:

```bash
nix run . -- README.md [--ip 127.0.0.1] [--port 3000]
```

#### Directly from GitHub (no clone required)

You can also run the latest version of `mdwatch` directly from GitHub, without cloning the repository:

```bash
nix run github:santoshxshrestha/mdwatch -- README.md [--ip 127.0.0.1] [--port 3000]
```

This will fetch, build, and run `mdwatch` from the official repository. You can pass any arguments after the `--` as usual.

### Enter a development environment

If you want to contribute or develop, enter a shell with all dependencies (Rust, cargo, etc) using:

```bash
nix develop
```

This gives you a reproducible environment with everything needed to build, test, and run `mdwatch`.

> **Note:**
>
> - You need Nix 2.4+ with flakes enabled. See the [Nix Flakes documentation](https://nixos.wiki/wiki/Flakes) for setup help.
> - On NixOS, flakes are usually enabled by default. On other systems, you may need to add `experimental-features = nix-command flakes` to your `~/.config/nix/nix.conf`.

---

## Uninstallation

You can uninstall using the provided script or manually:

### 1. Quick Uninstall via Script

```bash
curl -sSfL https://raw.githubusercontent.com/santoshxshrestha/mdwatch/main/scripts/uninstall.sh | bash
```

### 2. Manual Uninstall

Remove the binary from your PATH:

```bash
sudo rm /usr/local/bin/mdwatch
```

or

```bash
sudo rm /usr/bin/mdwatch
```

If you also want to remove your cloned repository:

```bash
rm -rf ~/mdwatch
```

If installed with Cargo:

```bash
cargo uninstall mdwatch
```

---

## Usage

Run the tool with the required and optional arguments:

```bash
mdwatch README.md [--ip 127.0.0.1] [--port 3000]

```

- `file`: Path to the Markdown file to preview (required)

- `--ip`: IP address to bind the server to (default: 127.0.0.1)

- `--port`: Port number for the server (default: 3000)

---

> [!NOTE]
> If you bind to `0.0.0.0`, the tool will warn you because this exposes the server to you local network.
> When binding to `0.0.0.0`, the preview URL opened in your browser will use `localhost` as the hostname.

---

# Example

```bash
mdwatch  notes.md --ip 0.0.0.0 --port 8080
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
