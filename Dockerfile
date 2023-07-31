FROM rust:1-bullseye AS build-image

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        wget \
        curl \
        libpq-dev \
        pkg-config \
        libssl-dev \
        clang \
        build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates

COPY . /one-shot-cli
WORKDIR /one-shot-cli

RUN sh install-golang.sh
ENV PATH=$PATH:/usr/local/go/bin

RUN cargo build --release -p one-shot-cli

FROM alpine:3.17.3 as alpine
RUN set -x \
    && apk update \
    && apk add --no-cache upx dumb-init
COPY --from=build-image /one-shot-cli/target/release/one-shot-cli /one-shot-cli/target/release/one-shot-cli
RUN upx --overlay=strip --best /one-shot-cli/target/release/one-shot-cli

FROM gcr.io/distroless/cc AS runtime
COPY --from=build-image /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=build-image /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build-image /etc/passwd /etc/passwd
COPY --from=build-image /etc/group /etc/group
COPY --from=alpine /usr/bin/dumb-init /usr/bin/dumb-init
COPY --from=alpine "/one-shot-cli/target/release/one-shot-cli" "/usr/local/bin/one-shot-cli"
ENTRYPOINT [ "/usr/bin/dumb-init", "--", "/usr/local/bin/one-shot-cli" ]
