FROM mcr.microsoft.com/dotnet/sdk:8.0.100-jammy

RUN apt-get update && apt-get install -y \
    bash                                 \
    git                                  \
    curl                                 \
    nodejs                               \
    npm                                  \
    golang-go                            \
    build-essential libssl-dev pkg-config\
    glibc-source                         \
    ca-certificates                      \
    make

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-wasi

# Install the gopls Go Language Server, see https://github.com/golang/tools/tree/master/gopls
RUN go install golang.org/x/tools/gopls@latest

# Install Spin and required plugins
RUN curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash && mv spin /usr/local/bin/
RUN spin plugin install js2wasm -y
RUN spin plugin install -y -u https://raw.githubusercontent.com/chrismatteson/spin-plugin-k8s/main/k8s.json

# # rust
# RUN /root/.cargo/bin/cargo install wasm-tools \
#     && /root/.cargo/bin/cargo install cargo-component \
#     && /root/.cargo/bin/cargo install cargo-shear

## https://github.com/dotnet/aspire/blob/main/docs/using-latest-daily.md
RUN dotnet workload update --skip-sign-check --source https://pkgs.dev.azure.com/dnceng/public/_packaging/dotnet8/nuget/v3/index.json \
    && dotnet workload install aspire --skip-sign-check --source https://pkgs.dev.azure.com/dnceng/public/_packaging/dotnet8/nuget/v3/index.json

# RUN dotnet workload install aspire