FROM rust:1.37.0-buster

RUN rustup install nightly
RUN rustup default nightly

RUN apt-get update \
  && apt-get install -y git libgtk-3-dev

# for wasm
RUN apt-get install -y nodejs npm gcc g++ gcc-arm-none-eabi
RUN npm install -g n
RUN n 10.15.1
RUN cargo install wasm-pack
RUN rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
RUN rustup run nightly rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf

RUN mkdir /work
WORKDIR /work

CMD ["/bin/sh"]