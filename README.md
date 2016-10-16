# `rush`

A ridiculously simple shell in Rust.

This is just a proof of concept, and I probably won't be developing it
further.

## Usage

To build, run `cargo build`, then run `./target/debug/rush` to try it
out.

## Features

Not a lot, since this is just a POC, but `rush` supports the following:

  - Running commands (kinda essential for a shell)
  - Quoting multi-word arguments (eg. `printf %s\n 'Hello "world"' "Hello 'world'" "'Hello'"' "world"'`)

`rush` doesn't support:

  - Backslash escaping
  - Variables
  - Builtins
  - Pretty much every other feature required to be POSIX compliant
