# 🔐 Paword – Rust Password Generator CLI

A fast and simple **command-line password generator** written in Rust.
Generate secure random passwords directly from your terminal.

Built using Rust, Clap, and Rand.

---

## 📦 Installation

Install using Cargo:

```
cargo install paword
```

---

## 🚀 Usage

Generate a password with the default length (12):

```
paword
```

Example output:

```
Generated Password: K9@dLm2QxP#f
```

Generate a password with a custom length:

```
paword -l 16
```

or

```
paword --length 16
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

```
paword -l 24
```

---

## ✨ Features

* 🔐 Generate secure random passwords
* ⚡ Fast and lightweight CLI tool
* 🦀 Written in Rust
* 🖥️ Works directly in the terminal
* 📦 Installable via Cargo

---

## 🛠 Built With

* Rust
* Clap – CLI argument parsing
* Rand – random number generation

---

## 📚 Documentation

Crate page:

```
https://crates.io/crates/paword
```

Documentation:

```
https://docs.rs/paword
```

---

## 🧑‍💻 Development

Clone the repository:

```
git clone https://github.com/CapThunder19/password-generator-rust.git
cd password-generator-rust
```

Run locally:

```
cargo run
```

Run with custom length:

```
cargo run -- -l 16
```

---

## 📂 Project Structure

```
paword
├── src
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## 📜 License

MIT License.

---

## ⭐ Support

If you find this project useful, consider giving it a **star on GitHub**.
