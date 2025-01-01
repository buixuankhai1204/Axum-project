# build stage
FROM rust:latest as builder

WORKDIR /workspace

RUN apt-get update && apt-get install lld clang -y

COPY . .

RUN cargo build --release

# deploy stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get clean

# create workspace directory
WORKDIR /workspace

COPY scripts/run .
COPY settings settings

# copy app bin
COPY --from=builder /workspace/target/release/app .

# copy migration bin
#COPY --from=builder /workspace/target/release/migration .

# expose port
EXPOSE 3000

ENV APP_PROFILE dev

ENV RUST_LOG info

# run the app
ENTRYPOINT ["./run"]