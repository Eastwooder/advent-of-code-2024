check: clippy test
    nix flake check

clippy:
    cargo clippy

test:
    cargo nextest run

watch:
    bacon nextest
