# is_ready

`is_ready` is a self-contained program designed to wait until multiple addresses become 
accessible. Its purpose is to coordinate the startup of interconnected services, such as 
Docker containers that rely on one another. This tool does not rely on any external 
dependencies and can be executed as a standalone binary.

## Table of Contents
- [Why](#why)
- [Usage](#usage)
- [Examples](#examples)
- [Installation](#installation)
- [Contributing](#contributing)

## Why
There are alternative solutions like 
[`wait-for-it`](https://github.com/vishnubob/wait-for-it) and [`wait-for`](https://github.com/eficode/wait-for). The key differentiating factor of `is_ready` compared to 
alternative solutions is its ability to wait for the availability of multiple addresses, 
not only one. Finally, unlike other options, `is_ready` stands out as a self-contained binary, eliminating the need for any external dependencies.

## Usage
```
$ is_ready --help

Usage: is_ready [OPTIONS] --addr <ADDRESS> -- <COMMAND>...

Arguments:
  <COMMAND>...

Options:
  -t, --timeout <TIMEOUT>  [default: 30]
  -a, --addr <ADDRESS>
  -h, --help               Print help
  -V, --version            Print version

```

## Examples
```
$ is_ready --timeout 10 --addr github.com:80 --addr google.com:80 -- echo "The addresses are available"
Connected to google.com:80 successfully
Connected to github.com:80 successfully
The addresses are available
```

## Installation

TBD

## Contributing 

TBD