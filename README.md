<div align="center">

# rs-clear

English | [中文](./README-CN.md)

</div>

---

## 📝 background

one day i realized my d drive was running out of space. while cleaning up unused files manually, i found rust build caches took up a lot of room — especially from projects that i wasn’t actively working on anymore. after clearing those `target` folders, i freed up almost 50GB! so i decided to create this little tool to help quickly find and delete those rust compilation caches, making disk cleanup easier and faster.

---

## ✨ Features

* Recursively scan for `Cargo.toml` files
* Locate adjacent `target/` build folders
* Interactive CLI selection for deletion
* Optional `--all` flag to delete all without prompting
* Clean, colorized CLI output

---

## 📂 Installation

### Build from source

```sh
cargo install --path .
```

---

## 🚀 Usage

```sh
rs-clear [PATH] [OPTIONS]
```

### Arguments

* `PATH` (optional): root directory to scan. Defaults to current directory (`.`)

### Options

* `-a`, `--all` - delete all matched target/ folders without selection
* `-h`, `--help` - display help
* `-V`, `--version` - show version

### Examples

```sh
# interactively select target/ folders to delete in current directory
rs-clear

# delete all target/ folders under ./workspace without prompting
rs-clear ./workspace --all
```

---

## 🧰 Output Example

```
scanning directory: ./workspace
select target directories to delete:
[deleted] ./workspace/crate-a/target
[deleted] ./workspace/crate-b/target
[failed]  ./workspace/crate-c/target: permission denied
deleting: ./workspace/crate-c/target
done
```

---

## ⚒️ Dependencies

* [`clap`](https://crates.io/crates/clap) - command-line argument parsing
* [`inquire`](https://crates.io/crates/inquire) - interactive selection UI
* [`walkdir`](https://crates.io/crates/walkdir) - recursive file scanning
* [`clap-cargo`](https://crates.io/crates/clap-cargo) - cargo-like CLI styling

---

## 📍 Feedback & Contributions

This project was built with the assistance of AI. If you have suggestions or feedback, feel free to open an [issue](https://github.com/yuyayang02/rs-clear/issues) or contribute via PR.
