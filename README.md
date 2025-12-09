# Git Favorite Branch

Quickly manage git branches during intensive multi-tasking work environments

## Usage

You can always run `gfb <SUBCOMMAND> -h` to get the latest usage.

```
Tiny CLI for enforcing branching name strategy across multiple repositories

Usage: gfb <COMMAND>

Commands:
  add      Add current branch to favorites
  use      Switch to a different branch
  del      Remove favorite branch
  del-all  Delete all favorite branches
  branch   Print Branch Name
  new      Create a new branch that is named with a value for a given key
  ls       List all favorite branches
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## Install

### Using Cargo

```bash
cargo install gfb
```

### Using Homebrew (MacOS Only)

```bash
brew tap revent-studio/tap
brew install gfb
```

## Build

Currently we don't have a build available for multiple OS, so you will need to clone the repo and build it your self. No worries here is the guide to get it working on your machine.

**Requirements**

- Rust (latest)

Installation Instructions

1. Clone this repo

```bash
git clone git@github.com:vepr-ua/git-favorite-branch.git
```

2. Build

```bash
cargo build
```

3. With the help of cargo make your executable visible to your environment

```bash
cargo install --path .
```

4. Run the executable

```bash
gfb --help
```
