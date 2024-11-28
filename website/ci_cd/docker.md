# Using cocogitto with docker

Docker images for `cog` are available [here](https://github.com/cocogitto/cocogitto/pkgs/container/cog). 

## Usage

```shell
docker pull ghcr.io/cocogitto/cog:latest
docker run -t -v "$(pwd)":/app/ cog check
```

Note that you need to mount a volume pointing to your target directory for `cog` to be able to operate on your git log.  