FROM rust:1.86-alpine3.21 AS builder
WORKDIR /app/
RUN apk update && apk add --no-cache musl-dev libressl-dev
COPY ./ ./
RUN cargo install --path=crates/in-typ-bot/

FROM alpine:3.21
WORKDIR /app/
RUN apk update && apk add --no-cache curl xz
RUN curl -fsSL https://typst.community/typst-install/install.sh | sh
RUN mv /root/.typst/bin/typst /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/in-typ-bot /usr/local/bin/
ENTRYPOINT ["in-typ-bot"]
