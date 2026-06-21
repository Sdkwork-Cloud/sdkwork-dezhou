#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');

function run(command) {
  const result = spawnSync(command, { cwd: repoRoot, shell: true, stdio: 'inherit' });
  if (result.status !== 0) process.exit(result.status ?? 1);
}

const args = process.argv.slice(2);
const command = args[0];

switch (command) {
  case 'dev':
    run('pnpm sdk:build');
    run('pnpm --filter sdkwork-dezhou-pc dev');
    break;
  case 'build':
    run('pnpm sdk:build && pnpm typecheck && cargo build --workspace');
    break;
  case 'test':
    run('node --test scripts/*.test.mjs tests/contract/*.test.mjs apps/sdkwork-dezhou-pc/tests/*.test.mjs && cargo test --workspace');
    break;
  case 'check':
    run(
      'pnpm sdk:build && pnpm db:validate && pnpm api:check && pnpm check:pnpm-script-standard && pnpm check:pc-package-naming && pnpm check:agent-workflow-standard && pnpm typecheck && pnpm --filter sdkwork-dezhou-pc-core typecheck && pnpm --filter sdkwork-dezhou-pc-dashboard typecheck && pnpm --filter sdkwork-dezhou-pc-shell typecheck && pnpm format:rust:check',
    );
    break;
  case 'clean':
    run('cargo clean');
    break;
  default:
    console.error('[sdkwork-dezhou] Usage: node scripts/sdkwork-command.mjs <dev|build|test|check|clean>');
    process.exit(1);
}
