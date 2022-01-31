FROM rust:latest
WORKDIR .
COPY mkpm_server/ .
RUN cargo build --release
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=prod
ENV ROCKET_PORT=8000
ENV ROCKET_LOG=critical
EXPOSE 8000
CMD ["target/release/mkpm_server"]