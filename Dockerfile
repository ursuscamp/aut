FROM rust:1.80 AS builder
WORKDIR /usr/src/aut
COPY . .
RUN cargo install --path .

FROM ubuntu:22.04
COPY --from=builder /usr/local/cargo/bin/aut /usr/local/bin/aut
EXPOSE 5555
ENV AUT_USERS_FILE=/etc/aut/users.yml
CMD ["aut"]
