# override the default just command to show the list of available recipes
default:
  @just --list

# lint with clippy
clippy-release:
    cargo clippy --release -- -W warnings -W clippy::pedantic -W clippy::nursery -W clippy::all -W clippy::cargo -W clippy::cognitive_complexity -A clippy::cargo_common_metadata

# run all tests with nextest
test-all *ARGS: check-dependencies
    cargo nextest run --verbose {{ ARGS }}

# run all tests with nextest in watch mode
test-watch-all *ARGS: check-dependencies
    cargo watch -c -w . -x "nextest run --verbose {{ ARGS }}"

# build the project in release mode
build-release:
    cargo build --release --verbose

# validate that cargo-nextest is installed
check-dependencies:
    #!/bin/sh
    ! type cargo-nextest > /dev/null 2>&1 && {
      echo "carg-nextest is not installed. Aborting...";
      exit 1;
    }
    exit 0;
