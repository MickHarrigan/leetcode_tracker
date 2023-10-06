set positional-arguments

default: build-all tui

build-all:
    @echo 'Building lc_lib, lc_cli, and lc_tui!'
    @cargo build

build-lib:
    @echo 'Building lc_lib'
    @cd lc_lib/
    @cargo build -r

build-cli:
    @echo 'Building lc_cli'
    @cargo build -r --bin lc_cli

build-tui:
    @echo 'Building lc_tui'
    @cargo build -r --bin lc_tui

cli *args='':
    @cargo run --bin lc_cli -- "$@"

tui *args='':
    @cargo run --bin lc_tui -- "$@"
