FROM debian:bookworm-slim

RUN apt update && apt install -y --no-install-recommends ca-certificates adduser
RUN update-ca-certificates

# Copy our build
COPY target/release/report /usr/local/bin/report

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

ENTRYPOINT ["report"]
