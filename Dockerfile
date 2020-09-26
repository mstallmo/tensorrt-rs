FROM nvcr.io/nvidia/tensorrt:19.06-py3

# Download and install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

#Add Cargo executables to path
ENV PATH="/root/.cargo/bin:${PATH}"

# Check version
RUN cargo --version
