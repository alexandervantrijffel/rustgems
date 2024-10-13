test-all *ARGS: 
    #!/bin/sh
    ! type cargo-nextest > /dev/null 2>&1 && {
        echo "Make sure to install cargo-nextest";
        exit 1;
    }
    cargo nextest run --verbose {{ ARGS }}
