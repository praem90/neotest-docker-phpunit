# PHPUnit docker test
Run phpunit tests in the docker container. This is a helper lib for [ neotest-docker-phpunit ]( https://github.com/praem90/neotest-docker-phpunit.nvim ) but works standalone.

## Installation
Install from source
```zsh
git clone https://github.com/praem90/neotest-docker-phpunit.git
cd neotest-docker-phpunit
cargo build
```

Install from cargo
```zsh
cargo install neotest-docker-phpunit
```

## Usage
```zsh
neotest-docker-phpunit /path/to/php/file/folder --log-junit=path/to/the/result.xml --container=php --volume="docker/style/:volume/map" --standalone=false
```

## Args
 - `--container` name or id of the phpunit container
 - `--volume` Map work dir to the container like docker's volume mount. E.g., `host/path:docker/path`
 - `--standalone` Whether use docker compose or not. Default `false` means `docker compose` will be used
 - `--log-junit` Unit test results xml file. Unit test results will be stored in this file.

## TODO
 - [ ] Unit tests
 - [ ] Improve STDOUT
 - [x] Make the root_dir, container name and coompose as args
 - [x] Create a `neotest-docker-phpunit` adapter for the `neotest` plugin
