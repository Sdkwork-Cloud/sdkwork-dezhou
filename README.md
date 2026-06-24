# SDKWork Dezhou

SDKWork Texas Hold'em poker application root. Provides table, seat, and hand-history capabilities aligned with `../sdkwork-specs`.

## Active layout

| Path | Purpose |
| --- | --- |
| `apis/` | HTTP API contracts (open/app/backend) |
| `apps/sdkwork-dezhou-pc/` | PC browser React application root |
| `crates/` | Rust services, repositories, API server |
| `database/` | `sdkwork-database` lifecycle assets (`moduleId=dezhou`, prefix `dezhou_`) |
| `sdks/` | SDK families and route manifests |
| `scripts/`, `tools/` | Verification, generation, and command dispatch |
| `deployments/` | Deployment descriptors and packaging handoff |
| `configs/` | Safe runtime config templates |

## Framework integration

- **HTTP**: `sdkwork-web-framework` via `crates/sdkwork-dezhou-api-server`
- **Database**: `sdkwork-database` via `crates/sdkwork-dezhou-database-host` and `database/`
- **Utils**: `@sdkwork/utils` (TypeScript), `sdkwork-utils-rust` (Rust)
- **Discovery**: not integrated (no RPC services yet; add when split-service RPC is required)

## Commands

```bash
pnpm install
pnpm dev
pnpm verify
pnpm api:materialize
pnpm db:validate
```

See `AGENTS.md` and `../sdkwork-specs/README.md` for standards.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

