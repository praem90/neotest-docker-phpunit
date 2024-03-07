# PHPUnit docker test
Run phpunit tests in the docker container. This is a helper lib for (neotest-phpunit)[https://github.com/olimorris/neotest-phpunit] but works standalone. 

## Installation
Install from source
```zsh
git clone https://github.com/praem90/phpunit-docker-test.git
cd phpunit-docker-test
cargo build
```

## Configuration
Configure your neovim neotest-phpunit to use this binary instead of phpunit
```lua
require('neotest').setup({
    adapters = {
        require('neotest-phpunit')({
            phpunit_cmd = "{PATH_TO}/neotest-docker-phpunit/target/debug/neotest-docker-phpunit"
        }),
    }
})
```

## TODO
 - [ ] Make the root_dir, container name and coompose as args
 - [ ] Use this command only if the project uses the docker compose
 - [ ] Create a `neotest-docker-phpunit` adapter for the `neotest` plugin
