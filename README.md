<div align="center">
  <img src="./MpyC.png" alt="MpyC-Logo" width="100" height="100" style="display: inline-block; vertical-align: middle; margin-right: 10px;">
  <h1 style="display: inline-block; vertical-align: middle;">MpyC</h1>
</div>

### ⚡ MicroPython Cross Compiler Manager

> A fast, minimal Rust CLI to install, manage, and simplify `mpy-cross`.

MPyC is a developer-friendly wrapper around the official MicroPython cross compiler. It helps you install, manage, and use `mpy-cross` without remembering complex commands or paths.

## ✨ Why MPyC?

Working with `mpy-cross` directly can be repetitive:

* Manually downloading binaries
* Keeping track of versions
* Managing PATH configuration
* Remembering compile flags

MPyC streamlines all of that into a clean, modern CLI.

## 🚀 Features

* ⚡ One-command install for `mpy-cross` (`mpyc setup`)
* 🔄 Version management (`mpyc list` and `mpyc use`)
* 📦 Build wrapper for files and folders (`mpyc build`)
* 🩺 `doctor` command to validate your setup (`mpyc doctor`)
* 🎨 Clean help output and friendly UX
* 🦀 Built with Rust for speed and reliability

## 📦 Installation

### From Cargo (coming soon)

```bash
cargo install mpyc
```

### From Source

```bash
git clone https://github.com/yourname/mpyc.git
cd mpyc
cargo build --release
```

Binary will be located at:

```
target/release/mpyc
```

## 📖 Usage

### Show help

```bash
mpyc --help
```

### Setup MpyC for your system

```bash
mpyc setup
```

Installs `uv` and prepares the environment.

### List installed and available versions

```bash
mpyc list
```

### Use a specific version

```bash
mpyc use 1.17
```

### Check system status

```bash
mpyc doctor
```

### Build a Python file or folder

```bash
mpyc build -i src -o firmware
```

Enable verbose output:

```bash
mpyc build -i src -o firmware --verbose
```

## 🛠 Example Workflow

```bash
mpyc setup
mpyc build -i src -o firmware --verbose
```

No manual binary handling. No PATH headaches.

## 🧠 Philosophy

MPyC follows three principles:

1. Keep it simple
2. Make it fast
3. Stay out of the way

It doesn’t replace `mpy-cross` — it enhances it.

## 🔮 Roadmap

* Interactive TUI mode
* Auto-download correct platform binary
* Config file support
* Multi-project support
* Pre-build validation

## 🤝 Contributing

Contributions are welcome.

1. Fork the repo
2. Create a feature branch
3. Open a pull request

## 📜 License

MIT License

---

⚡ MPyC — Fast MicroPython compilation made simple.
