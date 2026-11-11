# Wiimote-OSC

A Wii Remote to OSC implementation created to control stage lights while dancing in the public.

## Prerequisite

- Follow build prerequisite for wiiuse-rs

## Build

- cargo build

## Run

Defaults OSC stream to `127.0.0.1:9000`:

```sh
cargo run
```

Choose a destination IP and UDP port:

```sh
cargo run -- -i 192.168.1.100 -p 9001
```

View all options:

```sh
cargo run -- --help
```
