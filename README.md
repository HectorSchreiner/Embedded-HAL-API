# Setting up the project
The following setup is based on this [Guide](https://blog.logrocket.com/complete-guide-running-rust-arduino/).

Install nightly
```bash
rustup toolchain install nightly
```
## Windows
Install Scoop
```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser # Needed to run a remote script the first time
irm get.scoop.sh | iex
```

```powershell
scoop install avr-gcc
scoop install avrdude
```
## Debian
```bash
sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
```
## Nixos
Install the dependencies and run it in a nix-shell with the command
```
nix develop
```

--- 
After any of these steps, the next step is to install the [ravedude](https://github.com/Rahix/avr-hal/blob/main/ravedude) tool for flashing the microcontroller board against `cargo`:
```bash
cargo +stable install ravedude
```

# Create a new project
```bash
cargo install cargo-generate
```

Instantiate this template, and follow the guide.
```bash
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

_N.B._, If there is an error when installing the `libudev-sys` crate, you will have to include it in your `cargo.toml` file under dependencies:
```toml
[dependencies]
libudev-sys = "0.1"
```
Consult the [libudev-sys repository](https://github.com/dcuddeback/libudev-sys) in case of further issues arising from `pkg-config`.

Now you can run `Cargo build` to build the project. This will create an `.ELF` file in `target/avr-atmega328p/debug/`

# Running the code
## Windows
Find the Serial Port under device manager, mine was here
![[assets/windows_serial_port.png]]
And compile the code with the command
```bash
cargo run -- -P COM3
```

## Linux
setting the serial com port for ravedude with this script:

```bash
export RAVEDUDE_PORT=/dev/ttyUSB0
```

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
