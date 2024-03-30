a template for writing NES games in Rust

includes an example game that uses sprites, background tiles, sfx, and controller input

## building (approximately true)
Pull the rust-mos image and open a terminal in the container:
```bash
docker pull mrkits/rust-mos
docker run -it --name rustmos --entrypoint bash -v rust-nes/hostfiles mrkits/rust-mos
```
or, if you use VSCode, you can open this repo as a dev container with the appropriate extension.

This docker image comes with rust nearly set up to do 6502 dev, but doesn't have all the mos targets defined.
You can build them in the container using the bootstrap [script](https://github.com/mrk-its/rust-mos/blob/master/src/bootstrap/bootstrap.py).
I've included the relevant target config (mos-nes-cnrom.json).

Add the mos-nes-cnrom target to cargo:
```bash
cargo build --target mos-nes-cnrom.json
```

Run the resulting binary at target/mos-nes-cnrom/game on an emulator like fceux

## attribution

* forked from https://github.com/kirjavascript/rust-nes-tmp
* linker help https://github.com/jgouly
* toolchain https://llvm-mos.org/wiki/Rust
