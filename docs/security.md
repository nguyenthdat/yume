# Security Model

## Security principles

- Backend is the only public control plane.
- Android never stores DeepSeek API keys.
- Android never calls OpenCode.
- OpenCode and Qdrant are private services.
- All logs are redacted even if prompt/response logging is enabled.
- User-visible answers must not reveal internals.

## Auth model

Yume uses Google OAuth / Gmail.

Flow:

```txt
Android Credential Manager / Google Sign-In
→ Google ID token
→ Backend /v1/auth/google
→ Backend verifies Google JWKS
→ Backend creates local user
→ Backend issues Yume JWT/session
```

Backend must verify:

- issuer,
- audience,
- expiration,
- `email_verified`,
- Google `sub`.

Use internal user IDs in the database, but only hashed user IDs in Qdrant/logs.

## Secret handling

Never ship in Android:

- DeepSeek API key.
- OpenCode server password.
- Qdrant credentials.
- backend internal service credentials.

Store server secrets in:

- `.env` for local dev,
- Kubernetes Secrets for staging/prod,
- external secret manager later if needed.

## Network policy

Public:

- backend ingress only.

Private:

- OpenCode Serve.
- Qdrant.
- Postgres.
- Redis.
- observability internal endpoints.

Kubernetes should use NetworkPolicy to restrict OpenCode/Qdrant access to backend only.

## Prompt and response logging

User allows prompt/response logging, but logs must be redacted.

Recommended flags:

```txt
YUME_LOG_PROMPTS=true
YUME_LOG_PROMPTS_REDACT=true
YUME_RETENTION_DAYS=30
```

Redact:

- API keys.
- OAuth/JWT/session tokens.
- Authorization headers.
- raw embeddings.
- internal user IDs.
- system prompts.
- tool metadata.

## Data retention

Default retention is 30 days, configurable.

Applies to:

- conversations,
- OCR chunks,
- uploaded documents,
- Qdrant points,
- prompt/response logs,
- agent run summaries.

## RAG leakage controls

Do not return to user or OpenCode:

- raw vector IDs,
- raw Qdrant point IDs,
- raw similarity scores,
- embeddings,
- internal user IDs,
- secret metadata.

## Agent/tool permissions

- Default deny.
- `mobile-chat` should not have web access directly.
- `deep-research` may have controlled public web access.
- `answer-reviewer` should not have web/tool access unless explicitly needed.
- Destructive tools are disallowed for MVP.

## Security checklist

- [ ] No DeepSeek key in Android repo/artifacts.
- [ ] No OpenCode public ingress.
- [ ] No Qdrant public ingress.
- [ ] Backend validates Google tokens.
- [ ] Backend enforces rate limits.
- [ ] Backend validates embedding vectors.
- [ ] Prompt snapshot harness passes.
- [ ] Secret scan passes.
- [ ] Logs are redacted.
- [ ] System prompt not exposed.
- [ ] Reasoning not exposed.
- [ ] Tool permissions locked.
