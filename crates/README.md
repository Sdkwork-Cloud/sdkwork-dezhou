# Crates

Rust workspace members:

| Crate | Responsibility |
| --- | --- |
| `sdkwork-dezhou-api-server` | HTTP bootstrap and router composition with `sdkwork-web-framework` |
| `sdkwork-dezhou-database-host` | `sdkwork-database` lifecycle bootstrap |
| `sdkwork-dezhou-table-service` | Texas Hold'em table domain service |
| `sdkwork-dezhou-table-repository-sqlx` | Table persistence adapter (memory and SQLx) |
| `sdkwork-router-health-app-api` | App health route manifest and router crate |
| `sdkwork-router-table-app-api` | App table route manifest and router crate |
| `sdkwork-router-table-backend-api` | Backend table route manifest and router crate |
