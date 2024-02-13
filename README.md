# **Packet**

Packet is an opinionated package manager and dependency management tool for python written in Rust.

## Installation

Make sure you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

1. Clone the repository:

```bash
git clone https://github.com/rumbleFTW/packet.git
```

2. Navigate to directory:

```bash
cd packet
```

3. Install the binary:

```bash
cargo install --path .
```

## Usage: packet <COMMAND>

#### Commands:

- `new`: Create a new project
- `run`: Run the current project
- `add`: Install modules to the current project
- `init`: Initialize a new project in the current directory
- `pull`: Install all dependencies from Packet.toml
- `activate`: Activate a project environment in the current shell session
- `help`: Print help or the help of the given subcommand(s)

#### Options:

`-h, --help`: Print help
`-V, --version`: Print version
