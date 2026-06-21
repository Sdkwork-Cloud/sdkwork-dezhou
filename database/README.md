# Dezhou Database

`sdkwork-database` lifecycle assets for module `dezhou` (`dezhou_` table prefix).

## Owner

- Team: dezhou-platform
- Module id: `dezhou`
- Service code: `DEZHOU`

## Engines

- Primary: PostgreSQL
- Dev/test: SQLite baseline parity

## Commands

```bash
pnpm db:validate
pnpm db:materialize:contract
pnpm db:plan
pnpm db:bootstrap
pnpm db:migrate
pnpm db:seed
pnpm db:drift:check
```

## Verification

Run `pnpm db:validate` after changing contract, baseline DDL, migrations, or seed manifests.
