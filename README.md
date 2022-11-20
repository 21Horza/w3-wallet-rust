Web3 Wallet in Rust
======
This projects shows how W3 works for building crypto Wallet that generates keys & public address. Besides, the wallet is able to send Eth to any other adress.

Setup & run
======

The .env file should contain the following:

```sh
GORLI=wss://goerli.infura.io/ws/v3/xxxxxxxxxxx
```
The ``GORLI`` value is an endpoint address for Görli testnet from https://www.infura.io/.

```sh
# Start
cargo run
```