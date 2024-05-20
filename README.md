1. Clone [`vrf` branch of Dojo Engine](https://github.com/reilabs/dojo/tree/vrf)
2. In `cairo-hint-vrf-demo`, run:
    ```
    cargo run --bin sozo --manifest-path path/to/dojo/Cargo.toml -- migrate apply
    cargo run --bin sozo --manifest-path path/to/dojo/dojo/Cargo.toml -- execute 0x3b70f42e8d91d321b762571377c076f8912972879915c2abcb5908af64d40ef spawn
    ```
3. Or test using:
    ```
    cargo run --bin sozo --manifest-path path/to/dojo/Cargo.toml -- test
    ```
