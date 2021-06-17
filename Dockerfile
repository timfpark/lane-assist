FROM rust:1.52 as build

COPY ./ ./

RUN mkdir -p /build
RUN cp target/armv7-unknown-linux-gnueabihf/release/lane-assist /build/

FROM ubuntu:18.04

COPY --from=build /build/lane-assist /

CMD ["/lane-assist"]
