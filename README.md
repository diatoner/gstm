# gstm

The Gist Manipulator

![Rust](https://github.com/four0000four/gstm/workflows/Rust/badge.svg)

### What is it?

`gstm` provides a command line interface for interacting with GitHub's
API for Gists. After using [gist.github.com](https://gist.github.com),
I wanted a composable tool to ease work from the terminal: 

```bash
gstm create \
  --public \
  --description "Utility script to save the world" \
  save_world.sh \
  README.md
```

### Foundation

`gstm` is based on the work at [alicanerdogan/gistr](https://github.com/alicanerdogan/gistr),
which is a great tool. 

### Installation

`gstm` is unreleased; so currently setup requires cloning the repo:

```bash
git clone https://github.com/four0000four/gstm.git && \
  cd gstm && \
  cargo build
```

### Usage

```
gstm 0.1.0
Tom <tom@fourzerofour.pw>
Gist manipulator

USAGE:
    gstm [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help         Prints help information
    -V, --version      Prints version information
    -v, --verbosity    Sets the level of verbosity

SUBCOMMANDS:
    create    Create a new Gist
    fork      Fork a Gist to make your own edits
    get       Retrieve the content of a single Gist
    help      Prints this message or the help of the given subcommand(s)
    list      Retrieve a listing of Gists
```

This is the output of `gstm --help`. Each subcommand has additional
flags and arguments, not detailed here.
