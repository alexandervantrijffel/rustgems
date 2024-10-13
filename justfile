build-release:
    cargo build --release --verbose

clippy-release:
    cargo clippy --release -- -W warnings -W clippy::pedantic -W clippy::nursery -W clippy::all -W clippy::cargo -A clippy::cargo_common_metadata

test-all *ARGS: 
    #!/bin/sh
    ! type cargo-nextest > /dev/null 2>&1 && {
        echo "Make sure to install cargo-nextest";
        exit 1;
    }
    cargo nextest run --verbose {{ ARGS }}
