# pdmake

pdmake is an in-development toolchain for Playdate development in Lua.

## Planned Features

- [ ] TOML based configuration
- [ ] Dependency management
    - [ ] Git repositories as a source
    - [ ] Convert standard Lua packages to be compatable with Playdate flavored Lua (`import` instead of `require`)
    - [ ] Asset dependencies
- [ ] Asset pipelines:
    - [ ] Aseprite
    - [ ] Others..?
- [ ] Lua toolchain:
    - [ ] Lua preprocessor based loosely on [nelua's preprocessor](https://nelua.io/overview/#preprocessor)
    - [ ] Transform relative paths in imports to absolute paths
    - [ ] Debug and Release builds
    - [ ] Unit testing
    - [ ] Playdate and Preprocessor aware formatting and linting
- [ ] Release management:
    - [ ] Generate `pdxinfo`
    - [ ] Automatically increment build number
    - [ ] CI pipeline integration
        - [ ] GitHub Actions
        - [ ] Others...?
