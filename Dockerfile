FROM rust:slim-bookworm AS build
WORKDIR /var/app/src
RUN echo "fn main() {}" >main.rs
WORKDIR /var/app
COPY Cargo.* ./
RUN cargo fetch
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /var/app/target/release/accsaber-playlist-generator /usr/bin/playlist-generator
EXPOSE 3030
CMD /usr/bin/playlist-generator
