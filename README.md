# PHPUnit docker test
Run phpunit tests in the docker container. This is a helper lib for (neotest-phpunit)[https://github.com/olimorris/neotest-phpunit] but works standalone.

## Installation
Install from source
```zsh
git clone https://github.com/praem90/neotest-docker-phpunit.git
cd neotest-docker-phpunit
cargo build
```

## Configuration
Configure your neovim neotest-phpunit to use this binary instead of phpunit
```lua
require('neotest').setup({
    adapters = {
        require('neotest-docker-phpunit')({
            phpunit_cmd = "{PATH_TO}/neotest-docker-phpunit",
            docker_phpunit = {
                "/home/user/projects/another/project" = {
                    container   = "phpunit-debug",
                    volume      = "/home/user/projects/another/project:/docker/work/dir"
                    standalone  = true,
                }
                default = {
                    container   = "phpunit",
                    volume      = "/source/dir:/docker/work/dir"
                    standalone  = false,
                    callback    = function (spec, args)
                        return spec
                    end
                }
            }
        }),
    }
})

```

## TODO
 - [x] Make the root_dir, container name and coompose as args
 - [ ] Unit tests
 - [x] Create a `neotest-docker-phpunit` adapter for the `neotest` plugin
