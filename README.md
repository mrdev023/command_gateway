# Project

## Requirements

- `protobuf` see [tonic dependencies](https://github.com/hyperium/tonic#dependencies)

## RUN LOCAL

- `docker build . -t gateway`
- `docker run -p 22:22 gateway`
- `ssh test@localhost "{\"identifier\": \"project_env\", \"token\": \"token\", \"command\": \"/bin/bash\", \"envs\": {}, \"args\": []}"`

## BOOK

```shell
cargo install mdbook
```

```shell
cd book/ && ~/.cargo/bin/mdbook watch
```