set windows-shell := ["pwsh.exe", "-c"]

build:
  cargo build

test:
  cargo test

[working-directory: "example"]
example: build
  ../target/debug/pdmake

