# Git Favorite Branch

Tiny CLI to check out the same branch in every repo without retyping the name. Save the branch you are on with `gfb add <key>`. In another repository, `gfb use <key>` checks that branch out, and `gfb new <key>` creates it with the same name. 

The keys you create are saved in a global file and become available to be reused on multiple git repositories.

**Case Example 1**

Say you are working on a ticket where you have a long descriptive branch name, i.e. `feat/JIRA-3413-connect-orders`. The work that you have to do is split into multiple microservices. You would leverage `gfb` to keep branches the same between all git repositories. 

```sh
# Start in frontend and create your initial work branch
cd frontend
git checkout -b feat/JIRA-3413-connect-orders

# Save branch with a short key
gfb add orders
# orders => feat/JIRA-3413-connect-orders

# Go to an order service to work on the feature
cd ../order-service
# Quickly create a new branch and switch to it
gfb new orders

# Choose another service repository
cd ../api
gfb new orders

# Now all the repositories that you have to touch
# to implement your feature all contain the same branch name.
```

**Case Example 2**

Your work requires long and well worded branch names that you can't always remember, but need to context switch frequently.

```sh
# At any point create your registry
gfb add orders feat/JIRA-3413-connect-orders
gfb add accounts feat/JIRA-2443-account-creation-and-deletion
gfb add bug1 fix/JIRA-8732-bug-users-getting-free-beer

# Recall if you forget your short keys 
gfb ls
# orders => feat/JIRA-3413-connect-orders
# accounts => feat/JIRA-2443-account-creation-and-deletion
# bug1 => fix/JIRA-8732-bug-users-getting-free-beer

# Quickly switch to one
gfb use accounts
```

## Usage

You can always run `gfb <SUBCOMMAND> -h` to get the latest usage.

```
Tiny CLI for enforcing branching name strategy across multiple git repositories

Usage: gfb <COMMAND>

Commands:
  add      Add current branch to favorites
  use      Switch to a different branch
  del      Remove favorite branch
  del-all  Delete all favorite branches
  branch   Print Branch Name
  new      Create a new branch that is named with a value for a given key
  ls       List all favorite branches
  rename   Rename a saved key
  sync     Syncronize another branch with the current one. Essentially perform a merge
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

### Using Homebrew

```bash
brew tap revent-studio/tap
brew install gfb
```

## Build

Currently we don't have a build available for multiple OS, so you will need to clone the repo and build it yourself. No worries here is the guide to get it working on your machine.

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
