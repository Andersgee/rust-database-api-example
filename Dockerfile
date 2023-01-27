FROM rust:1.66 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo run -p prisma-cli -- generate
RUN cargo install --path .


FROM debian:bullseye-slim
#get additional runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /usr/local/cargo/bin/rustapi /usr/local/bin/myapp
CMD ["myapp"]
