## Setup
1. clone this repository: `git clone git@github.com:reilabs/starknet-vrf.git`
2. `cd starknet-vrf`
3. clone all required dependencies:
   1. `git clone -b vrf git@github.com:reilabs/dojo.git`
   2. `git clone -b vrf git@github.com:reilabs/blockifier.git`
   3. `git clone -b vrf git@github.com:reilabs/scarb.git`
   4. `git clone -b 2.6.3-hints git@github.com:reilabs/cairo.git`
  
## Run sample project unit tests
1. `cd cairo-hint-vrf-demo`
2. `cargo run --bin sozo --manifest-path ../dojo/Cargo.toml -- test`

## Start Katana with Hints
1. `cd dojo`
2. `cargo run --bin katana -- --disable-fee`

## Deploy a contract with hints (after you started Katana in a new terminal tab):
1. `cd cairo-hint-vrf-demo`
2. `cargo run --bin sozo --manifest-path ../dojo/Cargo.toml -- build`
3. `cargo run --bin sozo --manifest-path ../dojo/Cargo.toml -- migrate apply`
4. `cargo run --bin sozo --manifest-path ../dojo/Cargo.toml -- execute 0x3b70f42e8d91d321b762571377c076f8912972879915c2abcb5908af64d40ef spawn`
