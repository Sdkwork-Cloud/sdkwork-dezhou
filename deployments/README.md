# Deployments

Profiles: `standalone`, `cloud`.

| Artifact | Purpose |
| --- | --- |
| `docker/Dockerfile.dezhou-api` | Container image for `sdkwork-dezhou-standalone-gateway` |
| `templates/docker-compose.cloud.example.yml` | Local cloud profile with PostgreSQL |
| `templates/server.env.example` | Server runtime env template |

No RPC split-service deployment yet. `sdkwork-discovery` integration is deferred until gRPC services are introduced.

See `../sdkwork-specs/DEPLOYMENT_SPEC.md`.
