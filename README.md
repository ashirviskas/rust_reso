Reso Rust
---
A clean implementation of [Reso](https://github.com/lynnpepin/reso) using Rust. The principle of Reso Rust is almost identical to [Reso](https://github.com/lynnpepin/reso). For the documentation and more info about the functionality, see `lynndotpy` work on the [official Reso implementation repo](https://github.com/lynnpepin/reso)

## Disclaimer
I made this project for learning purposes, it may not be maintained or updated, but a PR fixing or improving this implementation is always welcome.

It's made from scratch, no original code was looked at while developing, so some functionality may differ from the official `Reso` implementation. The official project may also include a rust implementation soon, be sure to check it out! 

## Build/run instructions
_instructions for linux only, but with minor changes it should work on other operating systems too_

To run compiled version:
`./reso_rust`

    USAGE:
        reso_rust [OPTIONS]

    OPTIONS:
        -f, --file <file>            Filepath to image to run the simulation on
        -h, --help                   Print help information
        -l, --last                   Only output the last step
        -n, --number <num_steps>     Number of steps to run the simulation for
        -o, --output <output_dir>    Directory to output the simulation steps to
        -V, --version                Print version information

To run debug version (~0.25s per step on new CPU)
1. Install Rust
2. `cargo run`


To build and run release version (~0.07s per step on a new CPU)
1. Install rust
2. Build project with `cargo build --release`
3. Run built binary by `./target/release/reso_rust`


## TODO
1. Add more todos
2. Compile to WebAssembly
3. Make interactive
4. Split logic into multiple files

## License
Project licensed under `MIT License`, see [LICENSE](LICENSE)

## reso.png attribution
file `reso.png` is under MIT license, Copyright © 2022 lynndotpy