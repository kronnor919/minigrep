# Minigrep

Minigrep is a basic implementation in Rust of the Linux utility [grep](https://es.wikipedia.org/wiki/Grep).

## Usage

```bash
minigrep word ~/path/to/file.txt

minigrep "pub struct" ~/path/to/file.rs
```

In the output, you will see the lines where the requested text appears.

### Case sensitive

By default, Minigrep will distinguish between uppercase and lowercase letters. If you don't want this, you can create the `IGNORE_CASE` environment variable; the value doesn't matter, it's enough that it exists.

```bash
# Linux
export IGNORE_CASE=1
```

## Installation

### Via `curl`

Run:

```bash
curl -sL <https://raw.githubusercontent.com/kronnor919/minigrep/main/install.sh> | sudo bash
```

### Manual

- Go to the latest [release](https://github.com/kronnor919/minigrep/releases/latest) on GitHub
- Scroll to the bottom
- Download the .tar.gz
- Unzip it using: `tar -xzf <filename>`
- Move it to a location recognized by the `PATH`. Example: `sudo mv <binary> /usr/local/bin/`
- Test it with `minigrep --version`
