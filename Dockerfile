ARG BASE_IMAGE=rust:1.59.0-buster

FROM ${BASE_IMAGE} as build

# create a new empty shell project
RUN USER=root cargo new --bin shoppinglist
WORKDIR /shoppinglist

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your sources
COPY ./src ./src
COPY ./.env ./.env
COPY ./shoppinglist.db ./shoppinglist.db

# build for release
# RUN rm ./target/release/deps/shoppinglist*
RUN cargo clean && \
    cargo build --release

# our final base
FROM discolix/cc:latest-linux_amd64
# FROM discolix/cc:latest-linux_arm64

WORKDIR /storage
WORKDIR /app

# copy the build artifact from the build stage
COPY --from=build /shoppinglist/target/release/shoppinglist .

# set the startup command to run your binary
CMD ["./shoppinglist"]