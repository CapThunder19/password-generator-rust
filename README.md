# 🔐 Pword – Rust Password Generator CLI

A fast and simple **command-line password generator** written in Rust.
Generate secure random passwords directly from your terminal.

Built with **Rust**, **Clap**, and **Rand**.

---

## ✨ Features

* 🔑 Generate secure random passwords
* ⚡ Fast and lightweight CLI tool
* 🖥️ Works directly from the terminal
* 🦀 Written in Rust for performance and safety

---

## 📦 Installation

### Install from crates.io

```bash
cargo install pword
```

### Build from source

```bash
git clone https://github.com/yourusername/pword.git
cd pword
cargo build --release
```

Run the binary:

```bash
./target/release/pword
```

---

## 🚀 Usage

Generate a password with default length (12):

```bash
pword
```

Example output:

```
Generated Password: K9@fLm2QxP#d
```

Generate a password with custom length:

```bash
pword -l 16
```

or

```bash
pword --length 16
```

Example output:

```
Generated Password: aF8@kL92!dXzQp7L
```

---

## ⚙️ CLI Options

| Option           | Description                      | Default |
| ---------------- | -------------------------------- | ------- |
| `-l`, `--length` | Length of the generated password | `12`    |

Example:

```bash
pword -l 20
```

---

## 🛠 Built With

* Rust
* Clap (CLI argument parsing)
* Rand (random number generation)

---

## 📂 Project Structure

```
pword
├── Cargo.toml
└── src
    └── main.rs
```

---

## 🧠 How It Works

1. Reads CLI arguments using Clap
2. Generates random indices using Rand
3. Selects characters from a predefined charset
4. Combines them into a password string
5. Prints the result in the terminal

---

## 🧑‍💻 Development

Run locally:

```bash
cargo run
```

Run with custom length:

```bash
cargo run -- -l 16
```

---

## 📜 License

MIT License

---

## 🤝 Contributing

Contributions are welcome!

If you'd like to improve the project:

1. Fork the repository
2. Create a new branch
3. Submit a Pull Request

---

## ⭐ Support

If you like this project, consider giving it a **star on GitHub**!
