FROM rust:1.68 as build

# create a new empty shell project
RUN USER=root cargo new --bin splunk-github-sbom
WORKDIR /splunk-github-sbom

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/splunk_github_sbom*
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc AS runtime

# copy the build artifact from the build stage
COPY --from=build /splunk-github-sbom/target/release/splunk-github-sbom .

# set the startup command to run your binary
ENTRYPOINT ["/splunk-github-sbom"]