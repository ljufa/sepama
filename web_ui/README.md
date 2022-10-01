## Install / check required tools
Make sure you have basic tools installed:

   - [Rust](https://www.rust-lang.org) 
     - Install: https://www.rust-lang.org/tools/install
   - [cargo-make](https://sagiegurari.github.io/cargo-make/)
     - Install: `$ cargo install cargo-make`
   - [microserver](https://github.com/robertohuertasm/microserver)
     - Install: `$ cargo install microserver`

## Run locally
1. Run proxy container (this is required to avoid cors errors when sending call to the backend)
    - Go to [mysepa-deployment](../mysepa-deployment)
    - `docker-compose up -d traefik`

2. Run UI app
   - Build `cargo make build_release`
   - Run `cargo make serve`
   - Open [http://localhost](http://localhost)