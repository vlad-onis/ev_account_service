FROM rust:1.66 as build

RUN apt-get update && apt-get install -y make autoconf automake libtool unzip
RUN apt-get install postgresql-client -y

# Install protoc (cf. http://google.github.io/proto-lens/installing-protoc.html)
ENV PROTOC_ZIP=protoc-3.13.0-linux-aarch_64.zip
RUN apt-get update && apt-get install -y unzip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v3.13.0/$PROTOC_ZIP \
    && unzip -o $PROTOC_ZIP -d /usr/local bin/protoc \
    && unzip -o $PROTOC_ZIP -d /usr/local 'include/*' \ 
    && rm -f $PROTOC_ZIP

# create a new empty shell project
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src
COPY ./build.rs ./build.rs
COPY ./configuration.yaml ./configuration.yaml
COPY ./protos ./protos

# Database setup for sqlx
ARG db_host="192.168.0.164"
ENV DB_HOST_ADDR=${db_host}
EXPOSE 50051

# # build for release
RUN cargo build --release


# our final base
FROM rust:1.66-slim-buster

# copy the build artifact from the build stage
COPY --from=build /app/target/release/ev_account_service .
COPY --from=build /app/configuration.yaml .

# set the startup command to run your binary
CMD ["./ev_account_service"]