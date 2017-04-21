# Contributing

## Overview

* all development has to be in the 'development' branch 
    or any other feature branch like:
    * bugfix/...
    * doc/...
    * feature/...

* All cargo tests have to be passed before pushing to the remote repo
    ```bash
    cargo test
    ```

* We enforce formating with cargo-fmt
    * before pushing to the remote repo (development) run `cargo fmt`
    
    ```bash
    cargo fmt
    ```
    After the `cargo fmt` call look over and rerun `cargo test` to check the result.
    Cargo fmt creates backup files `.bk` if something goes wrong, just delete the original file
    and remove the `.bs` from the backup file name.

    To clean up the backup files call something like:
    
    ```bash
    find . -type f -name '*.bk'
    ```

* Additional we suggest to run `cargo clippy` to find common programming errors
    
    ```bash
    cargo clippy --release --lib -- -Dclippy
    ```


# Details
## Cargo clippy installation

    ```bash
    git clone https://github.com/arcnmx/cargo-clippy.git
    cargo build --release
    cargo install
    ```

    
