set windows-shell := ["pwsh.exe", "-c"]

build:
  cargo build

test:
  cargo test

[working-directory: "example"]
example: build
  ../target/debug/pdmake build
  ../target/debug/pdmake run

[working-directory: "example"]
example-clean: build
  ../target/debug/pdmake clean
