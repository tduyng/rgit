# Build your own Git

[![progress-banner](https://backend.codecrafters.io/progress/git/410c1755-1cfe-48a9-bbdf-2463f8721df8)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

Welcome to the "Build Your Own HTTP Server" for Rust solutions!

This repository serves as a my Rust solutions to the
["Build Your Own Git" Challenge](https://codecrafters.io/challenges/git).

You'll build a small Git implementation that's capable of
initializing a repository, creating commits and cloning a public repository.
Along the way we'll learn about the `.git` directory, Git objects (blobs,
commits, trees etc.), Git's transfer protocols and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## Getting started

1. Ensure you have `cargo (1.54)` installed locally
1. Run `./your_git.sh` to run your Git implementation, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Testing locally

The `your_git.sh` script is expected to operate on the `.git` folder inside the
current working directory. If you're running this inside the root of this
repository, you might end up accidentally damaging your repository's `.git`
folder.

We suggest executing `your_git.sh` in a different folder when testing locally.
For example:

```sh
mkdir -p /tmp/testing && cd /tmp/testing
/path/to/your/repo/your_git.sh init
```

To make this easier to type out, you could add a
[shell alias](https://shapeshed.com/unix-alias/):

```sh
alias mygit=/path/to/your/repo/your_git.sh

mkdir -p /tmp/testing && cd /tmp/testing
mygit init
```
