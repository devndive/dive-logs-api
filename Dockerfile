FROM rust:latest AS builder

RUN update-ca-certificates

ENV USER=dive-logs-api
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistatnt" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /dive-logs-api

COPY ./ .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY ./data/dives.json /dive-logs-api/data/dives.json

WORKDIR /dive-logs-api

COPY --from=builder /dive-logs-api/target/release/dive-logs-api ./

USER dive-logs-api:dive-logs-api

CMD ["/dive-logs-api/dive-logs-api"]
