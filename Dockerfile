##########
# This stage builds the rust binaries
##########
FROM rust:1.73 as builder

ENV BASE_DIR=/redirect/

# Create a new empty package
WORKDIR $BASE_DIR
RUN cargo new --bin rust
WORKDIR $BASE_DIR/rust

# Copy over the manifests
COPY Cargo.* ./

# This build step will cache the dependencies
RUN cargo build --release

RUN rm ./src/*.rs

COPY src ./src
# Build binaries
RUN rm ./target/release/deps/redirect*

RUN cargo build --release

FROM --platform=linux/x86_64 debian:bookworm-slim as libs

FROM scratch 

COPY --from=libs /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/
COPY --from=libs /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/
COPY --from=libs /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/
COPY --from=libs /lib64/ld-linux-x86-64.so.2 /lib64/

COPY --from=builder  /redirect/rust/target/release/redirect /redirect/

CMD ["/redirect/redirect"]
