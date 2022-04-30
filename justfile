_default:    
    @just -l
test-cli:
    cargo test -p coral-cli

update-cli-snapshots:
    TRYCMD=overwrite cargo test -p coral-cli --test cli_tests