# Bootstrap

Application-root runtime composition for `sdkwork-dezhou-pc`.

- `environment.ts` resolves typed runtime config from manifest and Vite env keys.
- `sdkClients.ts` declares SDK family inventory metadata.
- `iamRuntime.ts` wires appbase IAM runtime and shared TokenManager.
- `runtime.ts` composes session, SDK clients, IAM, and domain providers.
- `routes.ts` registers shell route contributions for architecture verification.
