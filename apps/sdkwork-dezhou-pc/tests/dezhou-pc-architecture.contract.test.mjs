#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const pcRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

const requiredFiles = [
  'src/App.tsx',
  'src/AuthGate.tsx',
  'src/bootstrap/environment.ts',
  'src/bootstrap/iamRuntime.ts',
  'src/bootstrap/runtime.ts',
  'src/bootstrap/sdkClients.ts',
  'src/bootstrap/routes.ts',
  'config/browser/runtime-env.development.example.json',
  'config/browser/runtime-env.production.example.json',
  'specs/component.spec.json',
  'packages/sdkwork-dezhou-pc-shell/package.json',
];

test('sdkwork-dezhou-pc matches APP_PC_ARCHITECTURE required layout', () => {
  for (const relativePath of requiredFiles) {
    assert.ok(
      fs.existsSync(path.join(pcRoot, relativePath)),
      `missing required PC architecture file: ${relativePath}`,
    );
  }

  const appSource = fs.readFileSync(path.join(pcRoot, 'src/App.tsx'), 'utf8');
  assert.match(appSource, /AuthGate/u);
  assert.match(appSource, /createSdkworkDezhouPcRuntime/u);
  assert.doesNotMatch(appSource, /TableLobby/u, 'root App.tsx must stay thin');

  const componentSpec = JSON.parse(
    fs.readFileSync(path.join(pcRoot, 'specs/component.spec.json'), 'utf8'),
  );
  for (const entrypoint of componentSpec.contracts.runtimeEntrypoints) {
    assert.ok(
      fs.existsSync(path.join(pcRoot, entrypoint)),
      `component.spec runtimeEntrypoint must exist: ${entrypoint}`,
    );
  }
});
