# gstm

The Gist Manipulator

### What is it?

`gstm` provides a command line interface for interacting with GitHub's
API for Gists. After using [gist.github.com](https://gist.github.com),
I wanted a composable tool to ease work from the terminal. Plus, it's
a project to brush up on my Rust.

```bash
gstm create \
  --private \
  --description "Utility script to save the world" \
  save_world.sh \
  README.md
```

### Foundation

`gstm` is based on the work at [alicanerdogan/gistr](https://github.com/alicanerdogan/gistr),
which is a great tool. As it stands, I'd like to build on this tool
while incorporating other packages, perhaps including:
 - [github_auth](https://crates.io/crates/github_auth)

### Usage

You can see the `create` subcommand illustrated above; Soon installation,
compilation, and documentation will be included in this README.
