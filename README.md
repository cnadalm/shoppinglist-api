# README


## Build

### AMD64
cargo build --release

### AARCH64
cargo build --target aarch64-linux-gnu-gcc --release

### Docker
docker build -t shoppinglist .


## Deploy to Raspberry PI

Execute the shell script for building and deploying the artifact into the PI device

`./deploy`
