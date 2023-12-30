FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin splunk-github-sbom
WORKDIR /splunk-github-sbom

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# fixing the issue with getting OOMKilled in BuildKit
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/splunk_github_sbom*
RUN cargo build --release

# our final base, need to use the debian base because of the openssl dependency
FROM gcr.io/distroless/cc-debian12 AS runtime

# copy the build artifact from the build stage
COPY --from=build /splunk-github-sbom/target/release/splunk-github-sbom .

# set the startup command to run your binary
ENTRYPOINT ["/splunk-github-sbom"]