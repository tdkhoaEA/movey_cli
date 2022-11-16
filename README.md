<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [cli](#cli)
  - [Installation](#installation)
  - [run](#run)
  - [Create alias](#create-alias)
  - [run to get the output of cli](#run-to-get-the-output-of-cli)
  - [Output](#output)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# cli
Resource: https://docs.rs/clap/2.34.0/clap/struct.ArgMatches.html

## Installation
Binaries are automatically uploaded to GitHub for each version.

## run
```shell
cargo run --bin cli 
```

## Create alias
alias movey-cli = '../movey-cli/target/debug/cli' (Path to the binary code)

** Check target/debug/cli to get the path of the binary file **

## run to get the output of cli
```shell
 movey-cli -h
```

## Output

```shell
cli 0.1.0
East Agile


USAGE:
    cli

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
