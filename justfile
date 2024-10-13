build-release:
    cargo build --release --verbose

clippy-release:
    cargo clippy --release -- -Wwarnings -Wclippy::pedantic -Wclippy::nursery -Wclippy::all -Wclippy::cargo

test-all *ARGS: 
    #!/bin/sh
    ! type cargo-nextest > /dev/null 2>&1 && {
        echo "Make sure to install cargo-nextest";
        exit 1;
    }
    cargo nextest run --verbose {{ ARGS }}
