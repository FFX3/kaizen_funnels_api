FROM debian:bullseye-slim AS server
RUN apt-get update && \
    apt-get install -y wget postgresql && \
    rm -rf /var/lib/apt/lists/*
    
RUN wget https://dl.google.com/cloudsql/cloud_sql_proxy.linux.amd64 -O cloud_sql_proxy
RUN apt-get remove -y wget
RUN chmod +x ./cloud_sql_proxy

FROM rust:1.67 as builder
WORKDIR /opt
RUN USER=root cargo new --bin server
WORKDIR /opt/server
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm ./src/*.rs
RUN rm ./target/release/deps/rocket*
ADD ./src ./src
ADD ./migrations ./migrations

RUN cargo build --release

FROM server as final
COPY --from=builder /opt/server/target/release/rocket /opt/server/rocket
EXPOSE 8000
ADD ./funnel-cms-2cd491b58fac.json ./credential_file.json
ADD ./.env ./.env
CMD ./cloud_sql_proxy -instances=funnel-cms:us-central1:main=tcp:5432 -credential_file credential_file.json &>/dev/null & /opt/server/rocket
