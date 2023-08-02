FROM rust:slim-bookworm AS build
WORKDIR /var/app/src
RUN echo "fn main() {}" >main.rs
WORKDIR /var/app
COPY Cargo.* ./
RUN cargo fetch
RUN apt-get update
RUN apt-get -y install libssl-dev openssl pkg-config
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update
RUN apt-get -y install openssl
COPY --from=build /var/app/target/release/accsaber-playlist-generator /usr/bin/playlist-generator
EXPOSE 3030
CMD /usr/bin/playlist-generator
