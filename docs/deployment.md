# Deployment Design

## Environments

- Local/dev: Docker Compose.
- Staging: Kubernetes.
- Production: Kubernetes.

Data location is local/self-hosted Docker/Kubernetes.

## Docker Compose dev stack

Services:

```txt
backend
qdrant
opencode
mock-deepseek
mock-opencode
postgres
redis
prometheus
```

Expected command:

```txt
make dev
```

Dev modes:

- mock DeepSeek by default,
- real DeepSeek opt-in through env,
- mock OpenCode for backend tests,
- real OpenCode for agent E2E.

## Kubernetes staging/prod

Recommended resources:

- Backend: Deployment + HPA.
- Qdrant: StatefulSet + PVC.
- OpenCode: Deployment, ClusterIP only.
- Postgres: managed or StatefulSet for local/self-hosted.
- Redis: rate limit and ephemeral coordination.
- Observability: Prometheus/Grafana/Loki or equivalent.

Ingress exposes only backend.

Private services:

- OpenCode.
- Qdrant.
- Postgres.
- Redis.

## OpenCode deployment

Run:

```txt
opencode serve --hostname 0.0.0.0 --port 4096
```

But expose as ClusterIP only.

Set:

```txt
OPENCODE_SERVER_USERNAME=opencode
OPENCODE_SERVER_PASSWORD=...
```

Production should pin OpenCode version or image digest even if development tracks latest.

## Backend configuration

Important env vars:

```txt
YUME_ENV=production
YUME_RETENTION_DAYS=30
YUME_LOG_PROMPTS=true
YUME_LOG_PROMPTS_REDACT=true
YUME_PUBLIC_BASE_URL=https://api.example.com
YUME_JWT_ISSUER=yume
YUME_GOOGLE_ANDROID_CLIENT_ID=...
DEEPSEEK_API_KEY=...
OPENCODE_BASE_URL=http://opencode:4096
OPENCODE_SERVER_USERNAME=opencode
OPENCODE_SERVER_PASSWORD=...
QDRANT_URL=http://qdrant:6334
DATABASE_URL=postgres://...
REDIS_URL=redis://...
```

## Observability

Use:

- OpenTelemetry traces.
- Prometheus metrics.
- Structured JSON logs.
- Redaction layer before log sink.

Trace IDs should propagate across:

```txt
Android request
→ Backend
→ Qdrant
→ OpenCode
→ DeepSeek
```

## Backup/restore

Production needs backups for:

- Postgres.
- Qdrant volumes/snapshots.
- OpenCode config/agents/skills.

Retention cleanup must not replace backups; backup lifecycle should separately enforce retention.
