FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src src
RUN touch src/main.rs
RUN cargo build --release

RUN strip target/release/report

FROM debian:bookworm-slim AS release
WORKDIR /app

COPY --from=builder /app/target/release/report .

RUN apt update && apt install -y --no-install-recommends ca-certificates adduser
RUN update-ca-certificates

# Create appuser
ENV USER=report_user
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Use an unprivileged user.
USER ${USER}:${USER}

ENTRYPOINT [ "./report" ]
