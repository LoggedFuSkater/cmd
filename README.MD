# LoggedFuSkater Command Line Tool
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![release](https://img.shields.io/github/v/release/LoggedFuSkater/cmd)](https://github.com/LoggedFuSkater/cmd/releases/latest)

See: https://github.com/LoggedFuSkater/standard/blob/master/README.MD

## Installation
### Build from source
 - Install rust: https://www.rust-lang.org/tools/install
 - Run `cargo build --release`
 - The executable is found in `{REPO}/target/release/`
 - Move the executable to your `bin/` of choice

 ## Usage
 Standard call:
 ```
> lfs "text goes here"
 StupidlyScaredPerson
 ```
With padding:
 ```
> lfs -p 3 "text goes here"
 StupidlyScaredPerson8CB0CB
 ```
By piping:
 ```
> echo "text goes here" | lfs
 StupidlyScaredPerson
 ```
Piping with padding:
 ```
> echo "text goes here" | lfs -p 3
 StupidlyScaredPerson8CB0CB
 ```
