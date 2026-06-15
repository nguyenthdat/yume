# Yume OpenCode Server — dedicated Docker image
# Builds the OpenCode CLI server with project-specific agents and skills.

FROM node:22-alpine

WORKDIR /app

# Install OpenCode CLI globally
RUN npm install -g @opencode-ai/cli

# Create a non-root user
RUN addgroup -g 1001 opencode && \
    adduser -u 1001 -G opencode -s /bin/sh -D opencode

# The config directory will be mounted as a volume at runtime
RUN mkdir -p /opencode-config && chown opencode:opencode /opencode-config

USER opencode

EXPOSE 4096

ENTRYPOINT ["opencode"]
CMD ["serve", "--hostname", "0.0.0.0", "--port", "4096", "--config", "/opencode-config"]
