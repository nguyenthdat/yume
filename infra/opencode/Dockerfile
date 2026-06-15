# Yume OpenCode Server — dedicated Docker image
# Installs OpenCode CLI via the official install script.
# Config mounted from ./opencode-server/ at runtime.

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y curl bash ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install OpenCode CLI (installs to ~/.opencode/bin/opencode)
RUN curl -fsSL https://opencode.ai/install | bash

# Add to PATH
ENV PATH="/root/.opencode/bin:${PATH}"

# Verify install
RUN opencode --version

# Create non-root user
RUN useradd -m -s /bin/bash opencode && \
    mkdir -p /home/opencode/.config/opencode && \
    cp -r /root/.opencode /home/opencode/ && \
    chown -R opencode:opencode /home/opencode

ENV PATH="/home/opencode/.opencode/bin:${PATH}"

USER opencode
WORKDIR /home/opencode

EXPOSE 4096

ENTRYPOINT ["/home/opencode/.opencode/bin/opencode"]
CMD ["serve", "--hostname", "0.0.0.0", "--port", "4096"]
