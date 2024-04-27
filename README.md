# is_ready

`is_ready` is a self-contained program designed to wait until multiple addresses become 
accessible. Its purpose is to coordinate the startup of interconnected services, such as 
Docker containers that rely on one another. This tool does not rely on any external 
dependencies and can be executed as a standalone binary.

## Table of Contents
- [Why](#why)
- [Examples](#examples)
- [Installation & Usage](#installation--usage)
  - [Use inside Docker](#use-inside-docker)
  - [Use as a Docker container](#use-as-a-docker-container)
- [License](#license)

## Why
There are alternative solutions like 
[`wait-for-it`](https://github.com/vishnubob/wait-for-it) and [`wait-for`](https://github.com/eficode/wait-for). The key differentiating factor of `is_ready` compared to 
alternative solutions is its ability to wait for the availability of multiple addresses, 
not only one. Finally, unlike other options, `is_ready` stands out as a self-contained binary, eliminating the need for any external dependencies.

## Examples
```bash
$ is_ready --help

Usage: is_ready [OPTIONS] --addr <ADDRESS> -- <COMMAND>...

Arguments:
  <COMMAND>...

Options:
  -t, --timeout <TIMEOUT>  [default: 30]
  -a, --addr <ADDRESS>
  -h, --help               Print help
  -V, --version            Print version


$ is_ready \
    --timeout 10 \
    --addr github.com:80 \
    --addr google.com:80 \
    -- echo "The addresses are available"

Connected to google.com:80 successfully
Connected to github.com:80 successfully
The addresses are available
```

## Installation & Usage

`is_ready` is optimized for running in a Docker container, and it is ideal to wait for the availability of other services such as databases, caches, or other services.

### Use inside Docker
Example using the Docker Image:

```Dockerfile
# Set the version you want to download
ARG IS_READY_VERSION=1.0.0
FROM ghcr.io/stavrospanakakis/is_ready:$IS_READY_VERSION AS is_ready

FROM alpine:3.19

RUN apk add --no-cache curl

# Download the binary
COPY --from=is_ready /is_ready /usr/local/bin/is_ready

...
...
```

Example using `curl`:
```Dockerfile
FROM alpine:3.19

RUN apk add --no-cache curl

# Set the version you want to download
ENV IS_READY_VERSION=v1.0.0

# Download the binary
RUN curl -L https://github.com/Stavrospanakakis/is_ready/releases/download/${IS_READY_VERSION}/is_ready_${IS_READY_VERSION}_x86_64-unknown-linux-musl.tar.gz | tar xzf - -C /usr/local/bin

...
...
```

Example Docker Compose:
```yaml
version: '3'
services:
  mysql:
    image: mysql:8.0
  
  app:
    build: .
    command: is_ready --timeout 10 --addr mysql:3306 -- echo "MySQL is ready"
```


### Use as a Docker container
`is_ready` can be also used as a Docker container using the 
[`ghcr.io/stavrospanakakis/is_ready`](https://github.com/Stavrospanakakis/is_ready/pkgs/container/is_ready) image from GitHub Container Registry.

```
$ VERSION=1.0.0
$ docker run --rm ghcr.io/stavrospanakakis/is_ready:$VERSION --version
is_ready 1.0.0
```

## License

MIT