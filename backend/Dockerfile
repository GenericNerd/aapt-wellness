FROM lukemathwalker/cargo-chef:latest-rust-1.74 AS chef
WORKDIR /usr/src/aapt-wellness-backend

FROM chef AS prepare
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=prepare /usr/src/aapt-wellness-backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM rust AS run
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=build /usr/src/aapt-wellness-backend/target/release/aapt-wellness-backend .
CMD ["./aapt-wellness-backend"]