FROM rust:latest

RUN cargo install --locked pueue

WORKDIR /workspace
COPY . .

CMD ["bash"]
