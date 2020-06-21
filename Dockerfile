FROM rust:latest-alpine as builder
WORKDIR /usr/src/prts_recruit
COPY . .
RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/prts_recruit /usr/local/bin/prts_recruit
CMD ["prts_recruit"]