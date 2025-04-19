# Vector

A lightweight, terminal-based text editor built in Rust, featuring syntax highlighting for Rust code.

## Download

You can download the latest release here:

[![Download for Linux ](https://img.shields.io/badge/Download-Linux-blue?style=for-the-badge&logo=linux)](https://github.com/user-attachments/files/19494030/vector.zip)

Extract the zip to a folder and export path
```sh
cd /{folder_name}
export PATH="/vector/bin:$PATH"
# now kill the terminal and open new terminal to reload .bashrc file
```

## Build from souce for mac/windows

Ensure you have Rust and Cargo installed. If not, install them using [[Rustup](https://rustup.rs/)](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and navigate to the project folder:

```sh
git clone https://github.com/xianglii-yao/vector.git
cd vector
```

## Running the Editor

Run the editor using Cargo:

```sh
cargo run -- release
```

To build an optimized release binary:

```sh
cargo build --release
./target/release/rust-text-editor
```

## Usage

- Open the editor in the terminal and start typing.
- Use common text navigation and editing shortcuts.
- Rust syntax highlighting is enabled automatically.

## Keyboard Shortcuts

- `Ctrl + S` - Save the file
- `Ctrl + X` - Quit the editor
- `Arrow Keys` - Move cursor
- `Ctrl + F` - Find text
- `Ctrl + Z` - Undo last action

## Preview

![Image](https://github.com/user-attachments/assets/5e1b34b7-94da-46e1-b118-53c1d622162c)

![Image](https://github.com/user-attachments/assets/446e3a01-a667-43ec-bc2a-0e05b22b6855)

## Contributing

Contributions are welcome! Feel free to fork the repository, create a new branch, and submit a pull request.

## Contact

For any questions or feedback, reach out via GitHub issues.
