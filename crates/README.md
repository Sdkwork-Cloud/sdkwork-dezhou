# Crates

Rust workspace members:

| Crate | Responsibility |
| --- | --- |
| `sdkwork-api-dezhou-standalone-gateway` | HTTP bootstrap and router composition with `sdkwork-web-framework` |
| `sdkwork-dezhou-database-host` | `sdkwork-database` lifecycle bootstrap |
| `sdkwork-dezhou-table-service` | Texas Hold'em table domain service |
| `sdkwork-dezhou-table-repository-sqlx` | Table persistence adapter (memory and SQLx) |
| `sdkwork-routes-health-app-api` | App health route manifest and router crate |
| `sdkwork-routes-table-app-api` | App table route manifest and router crate |
| `sdkwork-routes-table-backend-api` | Backend table route manifest and router crate |
