# Project

## Requirements

- `protobuf` see [tonic dependencies](https://github.com/hyperium/tonic#dependencies)

## RUN LOCAL

- `docker build . -t gateway`
- `docker run -p 22:22 gateway`
- `ssh test@localhost "{\"command\": \"/bin/bash\", \"envs\": {}, \"args\": []}"`