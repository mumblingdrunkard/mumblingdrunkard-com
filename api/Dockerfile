FROM rust:bullseye as api
WORKDIR /
RUN USER=root cargo new api
WORKDIR /api
COPY Cargo.* ./
RUN cargo build --release
RUN rm -rf src
COPY src src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bullseye-slim as production
WORKDIR /api/
COPY --from=api /api/target/release/api main
COPY test.html .
EXPOSE 8082
CMD ["/api/main"]
