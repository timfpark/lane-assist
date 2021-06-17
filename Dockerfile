# This Dockerfile assumes a cross compile step before this.
FROM ubuntu:18.04

COPY ./target/armv7-unknown-linux-gnueabihf/release/lane-assist ./

CMD ["/lane-assist"]
