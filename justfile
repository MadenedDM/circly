# Lints, Builds, and Tests
default: lint build test

alias pass := test
alias runs := server
alias runc := client
alias f := fmt
alias b := build
alias s := server
alias c := client

# Lints the code using clippy
lint: fmt
    cargo clippy

# Has the power to mess up your code, you should probably commit before using
apply-lint: fmt
    cargo clippy --fix --allow-dirty

# Formats the code using cargo fmt
fmt:
    cargo fmt --all -q

# # Creates a docker image of the server
# docker: 
#     docker build -t laurel .

# Runs the server
server:
    cargo run --bin laurel-server

# Runs the client
client:
    cargo run --bin laurel-client

# Builds for release
build: fmt
    cargo build --release

# Builds using the dev profile
dev-build: 
    cargo build

# Tests all targets
test:
    cargo test --all

# Runs cargo clean
clean:
    cargo clean

# Formats, Lints, Cleans
cleanup: fmt lint clean

# Formats, Applies lints, Cleans, has all the dangers of apply-lint
cleanup-ap: fmt apply-lint clean